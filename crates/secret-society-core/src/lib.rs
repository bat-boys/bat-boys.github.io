use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

pub const MAX_ABILITY_TOTAL: u32 = 30;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AbilityKey {
    pub category: String,
    pub target: String,
}

impl AbilityKey {
    pub fn label(&self) -> String {
        format!("{} of {}", self.category, self.target)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ability {
    pub key: AbilityKey,
    pub amount: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Level {
    pub number: usize,
    pub options: Vec<Ability>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Society {
    pub name: String,
    pub command: String,
    pub guild_masters: String,
    pub description: String,
    pub joining_abilities: Vec<String>,
    pub max_level: usize,
    pub levels: Vec<Level>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub line: Option<usize>,
    pub message: String,
}

impl ParseError {
    fn at_line(line: usize, message: impl Into<String>) -> Self {
        Self {
            line: Some(line),
            message: message.into(),
        }
    }

    fn general(message: impl Into<String>) -> Self {
        Self {
            line: None,
            message: message.into(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.line {
            Some(line) => write!(f, "line {}: {}", line, self.message),
            None => f.write_str(&self.message),
        }
    }
}

impl Error for ParseError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundledDataError {
    pub file: &'static str,
    pub source: ParseError,
}

impl fmt::Display for BundledDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.file, self.source)
    }
}

impl Error for BundledDataError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

/// Parses the society metadata and level options from a BatMUD society info file.
pub fn parse_society(input: &str) -> Result<Society, ParseError> {
    let mut name = None;
    let mut command = None;
    let mut guild_masters = String::new();
    let mut description_lines = Vec::new();
    let mut joining_abilities = Vec::new();
    let mut max_level = None;
    let mut levels: BTreeMap<usize, Vec<Ability>> = BTreeMap::new();
    let mut current_level = None;
    let mut in_description = false;
    let mut in_joining_abilities = false;

    for (line_index, raw_line) in input.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.trim();

        if let Some(value) = line.strip_prefix("Name:") {
            name = Some(value.trim().to_owned());
            in_description = false;
            in_joining_abilities = false;
            current_level = None;
            continue;
        }
        if let Some(value) = line.strip_prefix("Command:") {
            command = Some(value.trim().to_owned());
            continue;
        }
        if let Some(value) = line.strip_prefix("Guild masters:") {
            guild_masters = value.trim().to_owned();
            continue;
        }
        if let Some(value) = line.strip_prefix("Maximum level:") {
            max_level = Some(
                value
                    .trim()
                    .parse::<usize>()
                    .map_err(|_| ParseError::at_line(line_number, "invalid maximum level"))?,
            );
            continue;
        }
        if line == "Description:" {
            in_description = true;
            in_joining_abilities = false;
            current_level = None;
            continue;
        }
        if line == "Abilities gained when joining:" {
            in_description = false;
            in_joining_abilities = true;
            current_level = None;
            continue;
        }
        if line == "Abilities and requirements at each level:" {
            in_description = false;
            in_joining_abilities = false;
            current_level = None;
            continue;
        }
        if line.starts_with("Allows joining to the following guild:") {
            in_description = false;
            in_joining_abilities = false;
            current_level = None;
            continue;
        }

        if let Some(level_number) = parse_level_heading(line, line_number)? {
            if level_number == 0 {
                return Err(ParseError::at_line(line_number, "level numbers start at 1"));
            }
            if levels.insert(level_number, Vec::new()).is_some() {
                return Err(ParseError::at_line(
                    line_number,
                    format!("duplicate level {level_number}"),
                ));
            }
            current_level = Some(level_number);
            in_description = false;
            in_joining_abilities = false;
            continue;
        }

        if let Some(level_number) = current_level {
            if line.contains(" of ") && line.contains(" +") {
                let ability = parse_ability(line, line_number)?;
                levels
                    .get_mut(&level_number)
                    .expect("current level was just inserted")
                    .push(ability);
            }
        } else if in_description && !line.is_empty() {
            description_lines.push(line);
        } else if in_joining_abilities && !line.is_empty() {
            joining_abilities.push(line.to_owned());
        }
    }

    let name = name.ok_or_else(|| ParseError::general("missing Name field"))?;
    if name.is_empty() {
        return Err(ParseError::general("Name field is empty"));
    }
    let command = command.ok_or_else(|| ParseError::general("missing Command field"))?;
    if command.is_empty() {
        return Err(ParseError::general("Command field is empty"));
    }
    let max_level = max_level.ok_or_else(|| ParseError::general("missing Maximum level field"))?;
    if max_level == 0 {
        return Err(ParseError::general("Maximum level must be at least 1"));
    }

    let mut parsed_levels = Vec::with_capacity(max_level);
    for number in 1..=max_level {
        let options = levels
            .remove(&number)
            .ok_or_else(|| ParseError::general(format!("missing level {number}")))?;
        if options.is_empty() {
            return Err(ParseError::general(format!(
                "level {number} has no abilities"
            )));
        }
        parsed_levels.push(Level { number, options });
    }
    if let Some(unexpected_level) = levels.keys().next() {
        return Err(ParseError::general(format!(
            "level {unexpected_level} exceeds Maximum level {max_level}"
        )));
    }

    Ok(Society {
        name,
        command,
        guild_masters,
        description: description_lines.join("\n"),
        joining_abilities,
        max_level,
        levels: parsed_levels,
    })
}

fn parse_level_heading(line: &str, line_number: usize) -> Result<Option<usize>, ParseError> {
    let Some(number) = line
        .strip_prefix("Level ")
        .and_then(|line| line.strip_suffix(':'))
    else {
        return Ok(None);
    };
    let number = number
        .parse::<usize>()
        .map_err(|_| ParseError::at_line(line_number, "invalid level heading"))?;
    Ok(Some(number))
}

fn parse_ability(line: &str, line_number: usize) -> Result<Ability, ParseError> {
    let (category, target_and_amount) = line
        .split_once(" of ")
        .ok_or_else(|| ParseError::at_line(line_number, "expected ability 'of' target"))?;
    let (target, amount) = target_and_amount
        .rsplit_once(" +")
        .ok_or_else(|| ParseError::at_line(line_number, "expected ability amount"))?;
    let category = category.trim();
    let target = target.trim();
    if category.is_empty() || target.is_empty() {
        return Err(ParseError::at_line(
            line_number,
            "ability category and target must not be empty",
        ));
    }
    let amount = amount
        .parse::<u32>()
        .map_err(|_| ParseError::at_line(line_number, "invalid ability amount"))?;
    if amount == 0 {
        return Err(ParseError::at_line(
            line_number,
            "ability amount must be positive",
        ));
    }

    Ok(Ability {
        key: AbilityKey {
            category: category.to_owned(),
            target: target.to_owned(),
        },
        amount,
    })
}

const BUNDLED_SOCIETY_FILES: &[(&str, &str)] = &[
    ("lss.txt", include_str!("../data/lss.txt")),
    ("kabula.txt", include_str!("../data/kabula.txt")),
    ("maus.txt", include_str!("../data/maus.txt")),
    ("orion.txt", include_str!("../data/orion.txt")),
    ("sirius.txt", include_str!("../data/sirius.txt")),
    ("skull.txt", include_str!("../data/skull.txt")),
    ("skyforge.txt", include_str!("../data/skyforge.txt")),
    ("slf.txt", include_str!("../data/slf.txt")),
    ("war.txt", include_str!("../data/war.txt")),
    ("zhentarim.txt", include_str!("../data/zhentarim.txt")),
];

/// Parses the checked-in society input files in a stable order, with War+ first.
pub fn bundled_societies() -> Result<Vec<Society>, BundledDataError> {
    BUNDLED_SOCIETY_FILES
        .iter()
        .map(|(file, input)| {
            parse_society(input).map_err(|source| BundledDataError { file, source })
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evaluation {
    /// One option index or no selection for every society level.
    pub selections: Vec<Option<usize>>,
    /// Totals are keyed by ability category and target.
    pub totals: BTreeMap<AbilityKey, u32>,
    /// Whether each option is available under the aggregate cap.
    pub available_options: Vec<Vec<bool>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvaluationError {
    InvalidLevelCount {
        requested: usize,
        maximum: usize,
    },
    SelectionCountMismatch {
        expected: usize,
        actual: usize,
    },
    InvalidOptionIndex {
        level: usize,
        option: usize,
        option_count: usize,
    },
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLevelCount { requested, maximum } => write!(
                f,
                "visible level count {requested} is outside 1..={maximum}"
            ),
            Self::SelectionCountMismatch { expected, actual } => {
                write!(f, "expected {expected} level selections, received {actual}")
            }
            Self::InvalidOptionIndex {
                level,
                option,
                option_count,
            } => write!(
                f,
                "option {option} is invalid for level {level}, which has {option_count} options"
            ),
        }
    }
}

impl Error for EvaluationError {}

/// Recomputes totals and availability from the full selected-value state.
///
/// Existing selections are preserved. A selection set is valid when, for every
/// ability, the selected values before its latest selected row sum to less than
/// 30. The latest selected value may take the aggregate total above 30. An
/// unselected option is available when hypothetically choosing it preserves
/// that invariant.
pub fn evaluate_selection(
    society: &Society,
    visible_level_count: usize,
    requested_selections: &[Option<usize>],
) -> Result<Evaluation, EvaluationError> {
    if visible_level_count == 0 || visible_level_count > society.max_level {
        return Err(EvaluationError::InvalidLevelCount {
            requested: visible_level_count,
            maximum: society.max_level,
        });
    }
    if requested_selections.len() != society.max_level {
        return Err(EvaluationError::SelectionCountMismatch {
            expected: society.max_level,
            actual: requested_selections.len(),
        });
    }

    let selections = requested_selections.to_vec();
    let mut totals = BTreeMap::new();
    for (level_index, selected) in selections.iter().copied().enumerate() {
        let level = &society.levels[level_index];
        if let Some(option_index) = selected {
            let Some(ability) = level.options.get(option_index) else {
                return Err(EvaluationError::InvalidOptionIndex {
                    level: level.number,
                    option: option_index,
                    option_count: level.options.len(),
                });
            };
            *totals.entry(ability.key.clone()).or_insert(0) += ability.amount;
        }
        for ability in &level.options {
            totals.entry(ability.key.clone()).or_insert(0);
        }
    }

    let available_options = society
        .levels
        .iter()
        .take(visible_level_count)
        .enumerate()
        .map(|(level_index, level)| {
            level
                .options
                .iter()
                .enumerate()
                .map(|(option_index, _ability)| {
                    if selections[level_index] == Some(option_index) {
                        true
                    } else {
                        let mut hypothetical = selections.clone();
                        hypothetical[level_index] = Some(option_index);
                        selection_state_is_valid(society, &hypothetical)
                    }
                })
                .collect()
        })
        .collect();

    Ok(Evaluation {
        selections,
        totals,
        available_options,
    })
}

fn selection_state_is_valid(society: &Society, selections: &[Option<usize>]) -> bool {
    let mut totals = BTreeMap::new();
    let mut last_selected_values = BTreeMap::new();

    for (level_index, selected) in selections.iter().copied().enumerate() {
        let Some(option_index) = selected else {
            continue;
        };
        let Some(ability) = society.levels[level_index].options.get(option_index) else {
            return false;
        };
        *totals.entry(ability.key.clone()).or_insert(0) += ability.amount;
        last_selected_values.insert(ability.key.clone(), ability.amount);
    }

    totals.into_iter().all(|(key, total)| {
        total.saturating_sub(last_selected_values.get(&key).copied().unwrap_or(0))
            < MAX_ABILITY_TOTAL
    })
}

#[derive(Debug, Clone, Copy)]
struct StatChoice {
    level_index: usize,
    option_index: usize,
    amount: u32,
}

#[derive(Debug, Clone)]
struct StatOptimizationCandidate {
    selections: Vec<Option<usize>>,
    total: u32,
    final_amount: u32,
    final_level: usize,
    uses_second_largest: bool,
}

struct StatOptimizationContext<'a> {
    society: &'a Society,
    visible_level_count: usize,
    current: &'a [Option<usize>],
    target: &'a AbilityKey,
    second_largest: Option<u32>,
}

/// Adds an optimized selection for `target` without replacing the current picks.
///
/// It chooses a set of blank levels whose target-stat bonuses bring the total to
/// 29, then adds one remaining bonus that may take the total above 30. Among
/// exact 29-point combinations, it first avoids the second-largest available
/// bonus when possible. Existing selections are kept fixed.
pub fn optimize_stat_selection(
    society: &Society,
    visible_level_count: usize,
    current_selections: &[Option<usize>],
    target: &AbilityKey,
) -> Result<Vec<Option<usize>>, EvaluationError> {
    let initial_evaluation = evaluate_selection(society, visible_level_count, current_selections)?;
    let current = initial_evaluation.selections;
    if target.category != "Statmax" {
        return Ok(current);
    }
    let current_total = initial_evaluation.totals.get(target).copied().unwrap_or(0);
    if current_total >= MAX_ABILITY_TOTAL {
        return Ok(current);
    }

    let choices = (0..visible_level_count)
        .filter_map(|level_index| {
            if current[level_index].is_some() {
                return None;
            }
            society.levels[level_index]
                .options
                .iter()
                .enumerate()
                .find(|(_, ability)| &ability.key == target)
                .filter(|(option_index, _)| {
                    initial_evaluation.available_options[level_index][*option_index]
                })
                .map(|(option_index, ability)| StatChoice {
                    level_index,
                    option_index,
                    amount: ability.amount,
                })
        })
        .collect::<Vec<_>>();

    if choices.is_empty() {
        return Ok(current);
    }

    let mut available_amounts = choices
        .iter()
        .map(|choice| choice.amount)
        .collect::<Vec<_>>();
    available_amounts.sort_unstable_by(|left, right| right.cmp(left));
    let second_largest = available_amounts.get(1).copied();
    let context = StatOptimizationContext {
        society,
        visible_level_count,
        current: &current,
        target,
        second_largest,
    };

    let needed = MAX_ABILITY_TOTAL - 1 - current_total;
    let exact_without_second = collect_exact_candidates(&context, &choices, needed, second_largest);
    if let Some(best) = best_candidate(exact_without_second) {
        return Ok(best.selections);
    }

    let exact_with_second = collect_exact_candidates(&context, &choices, needed, None);
    if let Some(best) = best_candidate(exact_with_second) {
        return Ok(best.selections);
    }

    let mut fallback = Vec::new();
    for final_choice in choices.iter().copied() {
        let subtotal_choices = choices
            .iter()
            .copied()
            .filter(|choice| choice.level_index != final_choice.level_index)
            .collect::<Vec<_>>();
        let remaining = needed;
        let excluded = second_largest
            .and_then(|amount| best_subset_at_most(&subtotal_choices, remaining, Some(amount)));
        let unrestricted = best_subset_at_most(&subtotal_choices, remaining, None);
        let best_subset = match (excluded, unrestricted) {
            (Some(without), Some(with_any)) if without.0 >= with_any.0 => without,
            (_, Some(with_any)) => with_any,
            (Some(without), None) => without,
            (None, None) => (0, Vec::new()),
        };
        if let Some(candidate) =
            make_stat_candidate(&context, final_choice, &subtotal_choices, &best_subset.1)
        {
            fallback.push(candidate);
        }
    }

    Ok(best_candidate(fallback)
        .map(|candidate| candidate.selections)
        .unwrap_or(context.current.to_vec()))
}

fn collect_exact_candidates(
    context: &StatOptimizationContext<'_>,
    choices: &[StatChoice],
    needed: u32,
    excluded_amount: Option<u32>,
) -> Vec<StatOptimizationCandidate> {
    let mut candidates = Vec::new();
    for final_choice in choices.iter().copied() {
        let available_for_subtotal = choices
            .iter()
            .copied()
            .filter(|choice| choice.level_index != final_choice.level_index)
            .collect::<Vec<_>>();
        let Some(subset) = exact_subset(&available_for_subtotal, needed, excluded_amount) else {
            continue;
        };
        if let Some(candidate) =
            make_stat_candidate(context, final_choice, &available_for_subtotal, &subset)
        {
            candidates.push(candidate);
        }
    }

    candidates
}

fn exact_subset(
    choices: &[StatChoice],
    target: u32,
    excluded_amount: Option<u32>,
) -> Option<Vec<usize>> {
    let mut reachable = vec![None::<Vec<usize>>; target as usize + 1];
    reachable[0] = Some(Vec::new());

    for (choice_index, choice) in choices.iter().enumerate() {
        if excluded_amount == Some(choice.amount) || choice.amount > target {
            continue;
        }
        for sum in (0..=(target - choice.amount) as usize).rev() {
            if let Some(previous) = reachable[sum].clone() {
                let next_sum = sum + choice.amount as usize;
                if reachable[next_sum].is_none() {
                    let mut subset = previous;
                    subset.push(choice_index);
                    reachable[next_sum] = Some(subset);
                }
            }
        }
    }

    reachable[target as usize].clone()
}

fn best_subset_at_most(
    choices: &[StatChoice],
    maximum: u32,
    excluded_amount: Option<u32>,
) -> Option<(u32, Vec<usize>)> {
    let reachable = subset_sums(choices, maximum, excluded_amount);
    reachable
        .iter()
        .enumerate()
        .rev()
        .find_map(|(sum, subset)| subset.clone().map(|subset| (sum as u32, subset)))
}

fn subset_sums(
    choices: &[StatChoice],
    maximum: u32,
    excluded_amount: Option<u32>,
) -> Vec<Option<Vec<usize>>> {
    let mut reachable = vec![None::<Vec<usize>>; maximum as usize + 1];
    reachable[0] = Some(Vec::new());

    for (choice_index, choice) in choices.iter().enumerate() {
        if excluded_amount == Some(choice.amount) || choice.amount > maximum {
            continue;
        }
        for sum in (0..=(maximum - choice.amount) as usize).rev() {
            if let Some(previous) = reachable[sum].clone() {
                let next_sum = sum + choice.amount as usize;
                if reachable[next_sum].is_none() {
                    let mut subset = previous;
                    subset.push(choice_index);
                    reachable[next_sum] = Some(subset);
                }
            }
        }
    }

    reachable
}

fn make_stat_candidate(
    context: &StatOptimizationContext<'_>,
    final_choice: StatChoice,
    subtotal_choices: &[StatChoice],
    subset: &[usize],
) -> Option<StatOptimizationCandidate> {
    let mut proposed = context.current.to_vec();
    for choice_index in subset {
        let choice = subtotal_choices[*choice_index];
        proposed[choice.level_index] = Some(choice.option_index);
    }
    if proposed[final_choice.level_index].is_none() {
        proposed[final_choice.level_index] = Some(final_choice.option_index);
    }

    let evaluation =
        evaluate_selection(context.society, context.visible_level_count, &proposed).ok()?;
    if evaluation.selections != proposed {
        return None;
    }
    let total = evaluation.totals.get(context.target).copied().unwrap_or(0);
    let uses_second_largest = subset
        .iter()
        .any(|choice_index| context.second_largest == Some(subtotal_choices[*choice_index].amount));

    Some(StatOptimizationCandidate {
        selections: proposed,
        total,
        final_amount: final_choice.amount,
        final_level: final_choice.level_index,
        uses_second_largest,
    })
}

fn best_candidate(candidates: Vec<StatOptimizationCandidate>) -> Option<StatOptimizationCandidate> {
    candidates.into_iter().max_by(|left, right| {
        left.total
            .cmp(&right.total)
            .then_with(|| left.final_amount.cmp(&right.final_amount))
            .then_with(|| right.uses_second_largest.cmp(&left.uses_second_largest))
            .then_with(|| left.final_level.cmp(&right.final_level))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ability(category: &str, target: &str, amount: u32) -> Ability {
        Ability {
            key: AbilityKey {
                category: category.to_owned(),
                target: target.to_owned(),
            },
            amount,
        }
    }

    fn society_with_levels(level_options: Vec<Vec<Ability>>) -> Society {
        let max_level = level_options.len();
        Society {
            name: "Test Society".to_owned(),
            command: "test+".to_owned(),
            guild_masters: String::new(),
            description: String::new(),
            joining_abilities: Vec::new(),
            max_level,
            levels: level_options
                .into_iter()
                .enumerate()
                .map(|(index, options)| Level {
                    number: index + 1,
                    options,
                })
                .collect(),
        }
    }

    fn selected_targets(society: &Society, selections: &[Option<usize>]) -> Vec<String> {
        selections
            .iter()
            .enumerate()
            .map(|(level_index, selection)| match selection {
                Some(option_index) => society.levels[level_index].options[*option_index]
                    .key
                    .target
                    .clone(),
                None => "--".to_owned(),
            })
            .collect()
    }

    #[test]
    fn all_bundled_societies_parse_with_contiguous_levels() {
        let societies = bundled_societies().expect("bundled society data should parse");

        assert_eq!(societies.len(), 10);
        assert!(societies.iter().any(|society| society.command == "war+"));
        for society in &societies {
            assert_eq!(
                society.levels.len(),
                society.max_level,
                "{}",
                society.command
            );
            for (index, level) in society.levels.iter().enumerate() {
                assert_eq!(level.number, index + 1, "{}", society.command);
                assert!(
                    !level.options.is_empty(),
                    "{} level {}",
                    society.command,
                    level.number
                );
            }
        }
    }

    #[test]
    fn parser_handles_unindented_level_headers_and_multword_targets() {
        let input = "Name: Test\nCommand: test+\nMaximum level: 1\nDescription:\nExample\nAbilities and requirements at each level:\nLevel 1:\n Abilities:\n  Skillmax of martial arts +2\n";
        let society = parse_society(input).expect("valid input should parse");

        assert_eq!(society.description, "Example");
        assert_eq!(society.levels[0].options[0].key.target, "martial arts");
        assert_eq!(society.levels[0].options[0].amount, 2);
    }

    #[test]
    fn summary_sums_selected_options_and_allows_blank_levels() {
        let society = society_with_levels(vec![
            vec![ability("Statmax", "wis", 3), ability("Spellmax", "all", 1)],
            vec![ability("Statmax", "wis", 4)],
            vec![ability("Skillmax", "parry", 8)],
        ]);

        let result = evaluate_selection(&society, 3, &[Some(0), None, Some(0)])
            .expect("valid selection should evaluate");

        assert_eq!(result.selections, vec![Some(0), None, Some(0)]);
        assert_eq!(
            result.totals[&AbilityKey {
                category: "Statmax".into(),
                target: "wis".into(),
            }],
            3
        );
        assert_eq!(
            result.totals[&AbilityKey {
                category: "Skillmax".into(),
                target: "parry".into(),
            }],
            8
        );
        assert_eq!(
            result.totals[&AbilityKey {
                category: "Spellmax".into(),
                target: "all".into(),
            }],
            0
        );
    }

    #[test]
    fn selections_are_preserved_and_options_must_keep_the_last_row_valid() {
        let society = society_with_levels(vec![
            vec![ability("Statmax", "wis", 29)],
            vec![ability("Statmax", "wis", 10), ability("Statmax", "int", 3)],
            vec![ability("Statmax", "wis", 1), ability("Statmax", "int", 4)],
            vec![ability("Statmax", "wis", 2)],
        ]);

        let result = evaluate_selection(&society, 4, &[Some(0), Some(0), Some(1), None])
            .expect("selected choices should remain unchanged");

        assert_eq!(result.selections, vec![Some(0), Some(0), Some(1), None]);
        assert_eq!(
            result.totals[&AbilityKey {
                category: "Statmax".into(),
                target: "wis".into(),
            }],
            39
        );
        assert!(!result.available_options[2][0]);
        assert!(result.available_options[2][1]);
        assert!(!result.available_options[3][0]);
    }

    #[test]
    fn availability_uses_the_latest_selected_row_for_each_ability() {
        let society = society_with_levels(vec![
            vec![ability("Statmax", "wis", 29)],
            vec![ability("Statmax", "wis", 10), ability("Statmax", "int", 4)],
            vec![ability("Statmax", "wis", 10), ability("Statmax", "int", 4)],
            vec![ability("Statmax", "wis", 5), ability("Statmax", "int", 4)],
        ]);

        let first_order = evaluate_selection(&society, 4, &[Some(0), Some(0), None, None])
            .expect("first selection ordering should evaluate");
        let second_order = evaluate_selection(&society, 4, &[Some(0), None, Some(0), None])
            .expect("second selection ordering should evaluate");

        assert_eq!(first_order.totals, second_order.totals);
        assert_eq!(
            first_order.totals[&AbilityKey {
                category: "Statmax".into(),
                target: "wis".into(),
            }],
            39
        );
        assert!(!first_order.available_options[2][0]);
        assert!(!first_order.available_options[3][0]);
        assert!(!second_order.available_options[1][0]);
        assert!(!second_order.available_options[3][0]);
        assert!(first_order.available_options[0][0]);
        assert!(first_order.available_options[1][0]);
        assert!(second_order.available_options[2][0]);
    }

    #[test]
    fn last_selected_same_ability_is_removed_from_the_remaining_budget() {
        let society = society_with_levels(vec![
            vec![ability("Statmax", "wis", 29)],
            vec![ability("Statmax", "wis", 10)],
            vec![ability("Statmax", "wis", 5)],
        ]);

        let at_twenty_nine = evaluate_selection(&society, 3, &[Some(0), None, None])
            .expect("all options are available at a total of 29");
        assert!(at_twenty_nine.available_options[1][0]);
        assert!(at_twenty_nine.available_options[2][0]);

        let with_last_bonus = evaluate_selection(&society, 3, &[Some(0), Some(0), None])
            .expect("last-level bonus should remain selected");
        assert_eq!(
            with_last_bonus.totals[&AbilityKey {
                category: "Statmax".into(),
                target: "wis".into(),
            }],
            39
        );
        assert_eq!(with_last_bonus.selections, vec![Some(0), Some(0), None]);
        assert!(!with_last_bonus.available_options[2][0]);
    }

    #[test]
    fn subtotal_stays_valid_before_the_last_selected_row() {
        let society = society_with_levels(vec![
            vec![ability("Statmax", "wis", 3)],
            vec![ability("Statmax", "wis", 4)],
            vec![ability("Statmax", "wis", 5)],
            vec![ability("Statmax", "wis", 6)],
            vec![ability("Statmax", "wis", 8)],
            vec![ability("Statmax", "wis", 4)],
            vec![ability("Statmax", "wis", 5)],
            vec![ability("Statmax", "wis", 6)],
            vec![ability("Statmax", "wis", 8)],
            vec![ability("Statmax", "wis", 9)],
            vec![ability("Statmax", "wis", 5)],
        ]);
        let selections = [
            Some(0),
            Some(0),
            Some(0),
            Some(0),
            None,
            None,
            None,
            Some(0),
            None,
            None,
            Some(0),
        ];

        let result = evaluate_selection(&society, 11, &selections)
            .expect("the selected subtotal should be valid");
        let wis = AbilityKey {
            category: "Statmax".into(),
            target: "wis".into(),
        };

        assert_eq!(result.totals[&wis], 29);
        assert!(result.available_options[5][0]);
        assert!(result.available_options[6][0]);
        assert!(!result.available_options[8][0]);
        assert!(!result.available_options[9][0]);
    }

    #[test]
    fn overcap_options_must_precede_the_last_selected_row_for_that_ability() {
        let society = society_with_levels(vec![
            vec![ability("Statmax", "wis", 25)],
            vec![ability("Statmax", "wis", 4), ability("Statmax", "int", 2)],
            vec![ability("Statmax", "wis", 10)],
            vec![ability("Statmax", "wis", 4)],
        ]);

        let result = evaluate_selection(&society, 4, &[Some(0), None, Some(0), None])
            .expect("existing choices should remain selected");

        assert_eq!(
            result.totals[&AbilityKey {
                category: "Statmax".into(),
                target: "wis".into(),
            }],
            35
        );
        assert!(result.available_options[1][0]);
        assert!(result.available_options[1][1]);
        assert!(!result.available_options[3][0]);
    }

    #[test]
    fn cap_is_tracked_separately_by_ability_category_and_target() {
        let society = society_with_levels(vec![
            vec![
                ability("Statmax", "wis", 29),
                ability("Spellmax", "all", 29),
            ],
            vec![ability("Statmax", "wis", 1), ability("Spellmax", "all", 5)],
            vec![ability("Spellmax", "all", 1), ability("Skillmax", "all", 1)],
        ]);

        let result = evaluate_selection(&society, 3, &[Some(1), Some(1), Some(1)])
            .expect("each ability total has its own cap");

        assert_eq!(
            result.totals[&AbilityKey {
                category: "Spellmax".into(),
                target: "all".into(),
            }],
            34
        );
        assert_eq!(
            result.totals[&AbilityKey {
                category: "Skillmax".into(),
                target: "all".into(),
            }],
            1
        );
        assert!(!result.available_options[2][0]);
        assert!(result.available_options[2][1]);
    }

    #[test]
    fn war_plus_allows_twenty_nine_then_ten_for_thirty_nine() {
        let society = bundled_societies()
            .expect("bundled society data should parse")
            .into_iter()
            .find(|society| society.command == "war+")
            .expect("War+ should be bundled");
        // wis: +4, +5, +8, +4, +8 = 29 before level 15.
        let selections = [
            None,
            Some(0),
            Some(0),
            None,
            Some(0),
            Some(0),
            None,
            None,
            Some(0),
            None,
            None,
            None,
            None,
            None,
            Some(0),
        ];

        let result = evaluate_selection(&society, 15, &selections)
            .expect("the final +10 choice should be allowed");
        let wis = AbilityKey {
            category: "Statmax".into(),
            target: "wis".into(),
        };

        assert_eq!(result.totals[&wis], 39);
        assert_eq!(result.selections[14], Some(0));
    }

    #[test]
    fn stat_optimizer_matches_the_sirius_ten_level_sequence() {
        let society = bundled_societies()
            .expect("bundled society data should parse")
            .into_iter()
            .find(|society| society.command == "sirius+")
            .expect("Sirius+ should be bundled");
        let wis = AbilityKey {
            category: "Statmax".into(),
            target: "wis".into(),
        };
        let int = AbilityKey {
            category: "Statmax".into(),
            target: "int".into(),
        };

        let after_wis = optimize_stat_selection(&society, 10, &vec![None; society.max_level], &wis)
            .expect("Wis optimizer should produce a valid state");
        assert_eq!(
            &selected_targets(&society, &after_wis)[..10],
            [
                "wis", "wis", "wis", "wis", "--", "--", "wis", "wis", "--", "wis"
            ]
        );

        let after_int = optimize_stat_selection(&society, 10, &after_wis, &int)
            .expect("Int optimizer should preserve the Wis picks");
        assert_eq!(
            &selected_targets(&society, &after_int)[..10],
            [
                "wis", "wis", "wis", "wis", "int", "int", "wis", "wis", "int", "wis"
            ]
        );
        let totals =
            evaluate_selection(&society, 10, &after_int).expect("optimized state should evaluate");
        assert_eq!(totals.totals[&wis], 38);
        assert_eq!(totals.totals[&int], 20);
    }

    #[test]
    fn stat_optimizer_preserves_current_picks_and_leaves_second_value_when_possible() {
        let society = bundled_societies()
            .expect("bundled society data should parse")
            .into_iter()
            .find(|society| society.command == "sirius+")
            .expect("Sirius+ should be bundled");
        let wis = AbilityKey {
            category: "Statmax".into(),
            target: "wis".into(),
        };
        let int = AbilityKey {
            category: "Statmax".into(),
            target: "int".into(),
        };

        let after_wis = optimize_stat_selection(&society, 15, &vec![None; society.max_level], &wis)
            .expect("Wis optimizer should produce a valid state");
        assert_eq!(after_wis[9], None);
        assert_eq!(after_wis[13], None);
        let after_int = optimize_stat_selection(&society, 15, &after_wis, &int)
            .expect("Int optimizer should preserve the Wis picks");
        let totals =
            evaluate_selection(&society, 15, &after_int).expect("optimized state should evaluate");

        assert_eq!(totals.totals[&wis], 39);
        assert_eq!(totals.totals[&int], 38);
        for (level_index, selection) in after_wis.iter().enumerate() {
            if selection.is_some() {
                assert_eq!(after_int[level_index], *selection);
            }
        }
        // The second-largest Wis value (+9) stays free when Wis is optimized.
        // For Int, the level 10 +9 is needed to make an exact 29-point subtotal
        // before the level 14 +9 cap-crossing choice.
        assert_eq!(after_int[9], Some(0));
    }

    #[test]
    fn validates_level_count_and_option_indices() {
        let society = society_with_levels(vec![vec![ability("Statmax", "str", 3)]]);

        assert_eq!(
            evaluate_selection(&society, 0, &[]),
            Err(EvaluationError::InvalidLevelCount {
                requested: 0,
                maximum: 1,
            })
        );
        assert_eq!(
            evaluate_selection(&society, 1, &[Some(1)]),
            Err(EvaluationError::InvalidOptionIndex {
                level: 1,
                option: 1,
                option_count: 1,
            })
        );
    }
}
