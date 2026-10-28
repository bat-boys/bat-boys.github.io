use std::sync::Arc;

use bat_ui_components::{PageHeader, Panel, SummaryRow};
use leptos::prelude::*;
use secret_society_core::{bundled_societies, evaluate_selection, optimize_stat_selection};

#[component]
fn App() -> impl IntoView {
    let society_data = Arc::new(bundled_societies().expect("bundled society data must parse"));
    let first_level_count = society_data[0].max_level;
    let selected_society = RwSignal::new(0_usize);
    let visible_level_count = RwSignal::new(first_level_count);
    let selections = RwSignal::new(vec![None; first_level_count]);

    let society_options_data = Arc::clone(&society_data);
    let society_indices_data = Arc::clone(&society_data);
    let level_options_data = Arc::clone(&society_data);
    let level_counts_data = Arc::clone(&society_data);
    let metadata_data = Arc::clone(&society_data);
    let level_data = Arc::clone(&society_data);
    let summary_data = Arc::clone(&society_data);

    view! {
        <div id="app" class="ss-app">
            <main>
                <PageHeader title="Secret Society Calculator" />

                <div class="ss-layout">
                    <div class="ss-scroll-column">
                        <Panel>
                            <div class="ss-controls">
                                <label class="ss-field">
                                    <span>"Secret society"</span>
                                    <select
                                        class="pure-input-1"
                                        prop:value=move || selected_society.get().to_string()
                                        on:change=move |event| {
                                            let Ok(index) = event_target_value(&event).parse::<usize>() else {
                                                return;
                                            };
                                            if let Some(society) = society_data.get(index) {
                                                selected_society.set(index);
                                                visible_level_count.set(society.max_level);
                                                selections.set(vec![None; society.max_level]);
                                            }
                                        }
                                    >
                                        <For
                                            each={move || (0..society_indices_data.len()).collect::<Vec<_>>()}
                                            key=|index| *index
                                            children=move |index| {
                                                let society = &society_options_data[index];
                                                view! {
                                                    <option value=index.to_string()>
                                                        {format!("{}: {}", society.command, society.name)}
                                                    </option>
                                                }
                                            }
                                        />
                                    </select>
                                </label>

                                <label class="ss-field">
                                    <span>"Levels"</span>
                                    <select
                                        class="pure-input-1"
                                        prop:value=move || visible_level_count.get().to_string()
                                        on:change=move |event| {
                                            let Ok(count) = event_target_value(&event).parse::<usize>() else {
                                                return;
                                            };
                                            let society = &level_options_data[selected_society.get()];
                                            if count == 0 || count > society.max_level {
                                                return;
                                            }

                                            visible_level_count.set(count);
                                            let mut next = selections.get();
                                            next.resize(society.max_level, None);
                                            clear_hidden_selections(&mut next, count);
                                            if let Ok(evaluation) = evaluate_selection(society, count, &next) {
                                                selections.set(evaluation.selections);
                                            }
                                        }
                                    >
                                        <For
                                            each={move || {
                                                let maximum = level_counts_data[selected_society.get()].max_level;
                                                (1..=maximum).collect::<Vec<_>>()
                                            }}
                                            key=|count| *count
                                            children=move |count| view! {
                                                <option value=count.to_string()>{count}</option>
                                            }
                                        />
                                    </select>
                                </label>
                            </div>
                        </Panel>

                        <Panel title="Instructions".to_owned()>
                            <p class="instructions ss-cap-note">
                                "Select a society and the number of levels."
                            </p>
                            <p class="instructions ss-cap-note">
                                "The maximum stats is 29 + gain from the last selected level."
                            </p>
                            <p class="instructions ss-cap-note">
                                "Optimizer doesn't necessarily produce optimum results yet. All feedback is welcome, astrax @ BatMUD and discord."
                            </p>
                        </Panel>

                        <Panel title="Stats summary".to_owned()>
                            {move || {
                                let society = &summary_data[selected_society.get()];
                                let requested = selections.get();
                                let count = visible_level_count.get();
                                match evaluate_selection(society, count, &requested) {
                                    Ok(evaluation) => {
                                        let mut totals = evaluation
                                            .totals
                                            .into_iter()
                                            .filter(|(_, total)| *total > 0)
                                            .collect::<Vec<_>>();
                                        totals.sort_by(|(left, _), (right, _)| {
                                            compare_ability_keys(left, right)
                                        });
                                        let rows = totals
                                            .into_iter()
                                            .map(|(key, total)| {
                                                view! { <SummaryRow label=format_ability_label(&key) value=total /> }
                                            })
                                            .collect_view();
                                        rows.into_any()
                                    }
                                    Err(error) => view! { <p class="warning">{error.to_string()}</p> }.into_any(),
                                }
                            }}
                        </Panel>

                        <Panel title="Society information".to_owned()>
                            {move || {
                                let society = metadata_data[selected_society.get()].clone();
                                view! {
                                    <p>{society.name}</p>
                                    <p><strong>"Command: "</strong>{society.command}</p>
                                    <p><strong>"Guild masters: "</strong>{society.guild_masters}</p>
                                    {if society.description.is_empty() {
                                        ().into_any()
                                    } else {
                                        view! { <p class="ss-description">{society.description}</p> }.into_any()
                                    }}
                                    {if society.joining_abilities.is_empty() {
                                        ().into_any()
                                    } else {
                                        view! {
                                            <p><strong>"Abilities gained when joining:"</strong></p>
                                            <ul class="ss-joining-abilities">
                                                {society.joining_abilities.into_iter().map(|ability| view! { <li>{ability}</li> }).collect_view()}
                                            </ul>
                                        }.into_any()
                                    }}
                                }
                            }}
                        </Panel>
                    </div>

                    <div class="ss-scroll-column">
                        <Panel>
                            {move || {
                                let society = level_data[selected_society.get()].clone();
                                let count = visible_level_count.get();
                                let requested = selections.get();
                                match evaluate_selection(&society, count, &requested) {
                                    Ok(evaluation) => {
                                        let columns = ability_columns(&society);
                                        let headings = columns.iter().map(|key| {
                                            let label = format_ability_label(key);
                                            view! { <th scope="col">{label}</th> }
                                        }).collect_view();
                                        let optimize_buttons = columns.iter().map(|key| {
                                            if key.category == "Statmax" {
                                                let target = key.clone();
                                                let label = format!("Optimize {}", format_ability_label(key));
                                                let optimizer_societies = Arc::clone(&level_data);
                                                view! {
                                                    <td>
                                                        <button
                                                            class="pure-button ss-optimize-button"
                                                            type="button"
                                                            aria-label=label
                                                            on:click=move |_| {
                                                                let society = &optimizer_societies[selected_society.get()];
                                                                let current = selections.get();
                                                                if let Ok(optimized) = optimize_stat_selection(
                                                                    society,
                                                                    visible_level_count.get(),
                                                                    &current,
                                                                    &target,
                                                                ) {
                                                                    selections.set(optimized);
                                                                }
                                                            }
                                                        >
                                                            "optimize"
                                                        </button>
                                                    </td>
                                                }.into_any()
                                            } else {
                                                view! { <td class="ss-empty-cell"></td> }.into_any()
                                            }
                                        }).collect_view();
                                        let clear_all_societies = Arc::clone(&level_data);

                                        let rows = (0..count).map(|level_index| {
                                            let level = &society.levels[level_index];
                                            let selected = evaluation.selections[level_index];
                                            let show_empty_row = is_empty_before_last_selection(
                                                &evaluation.selections,
                                                level_index,
                                                count,
                                            );
                                            let group_name = format!("level-{}", level.number);
                                            let cells = columns.iter().map(|key| {
                                                match level.options.iter().enumerate().find(|(_, ability)| &ability.key == key) {
                                                    Some((option_index, ability)) => {
                                                        let disabled = !evaluation.available_options[level_index][option_index];
                                                        let checked = selected == Some(option_index);
                                                        let amount = ability.amount;
                                                        let label = format!(
                                                            "Select {} +{} at level {}",
                                                            format_ability_label(&ability.key),
                                                            amount,
                                                            level.number,
                                                        );
                                                        let deselect_societies = Arc::clone(&level_data);
                                                        let selection_societies = Arc::clone(&level_data);
                                                        view! {
                                                            <td class:ss-selected-cell=checked>
                                                                <label class:ss-unavailable=disabled class="ss-table-choice">
                                                                    <input
                                                                        type="radio"
                                                                        name=group_name.clone()
                                                                        value=option_index.to_string()
                                                                        aria-label=label
                                                                        prop:checked=checked
                                                                        prop:disabled=disabled
                                                                        on:click=move |_| {
                                                                            let mut proposed = selections.get();
                                                                            if proposed[level_index] == Some(option_index) {
                                                                                proposed[level_index] = None;
                                                                                let society = &deselect_societies[selected_society.get()];
                                                                                if let Ok(evaluation) = evaluate_selection(
                                                                                    society,
                                                                                    visible_level_count.get(),
                                                                                    &proposed,
                                                                                ) {
                                                                                    selections.set(evaluation.selections);
                                                                                }
                                                                            }
                                                                        }
                                                                        on:change=move |_| {
                                                                            let mut proposed = selections.get();
                                                                            proposed[level_index] = Some(option_index);
                                                                            let society = &selection_societies[selected_society.get()];
                                                                            if let Ok(evaluation) = evaluate_selection(
                                                                                society,
                                                                                visible_level_count.get(),
                                                                                &proposed,
                                                                            ) {
                                                                                selections.set(evaluation.selections);
                                                                            }
                                                                        }
                                                                    />
                                                                    <span>{amount}</span>
                                                                </label>
                                                            </td>
                                                        }.into_any()
                                                    }
                                                    None => view! { <td class="ss-empty-cell"></td> }.into_any(),
                                                }
                                            }).collect_view();

                                            let level_number = level.number;
                                            let clear_societies = Arc::clone(&level_data);
                                            view! {
                                                <tr>
                                                    <th scope="row">{level_number}</th>
                                                    {cells}
                                                    <td class="ss-clear-cell">
                                                        {if selected.is_some() {
                                                            view! {
                                                                <button
                                                                    class="pure-button ss-clear-level"
                                                                    type="button"
                                                                    aria-label=format!("Clear level {level_number}")
                                                                    on:click=move |_| {
                                                                        let mut next = selections.get();
                                                                        next[level_index] = None;
                                                                        let society = &clear_societies[selected_society.get()];
                                                                        if let Ok(evaluation) = evaluate_selection(
                                                                            society,
                                                                            visible_level_count.get(),
                                                                            &next,
                                                                        ) {
                                                                            selections.set(evaluation.selections);
                                                                        }
                                                                    }
                                                                >
                                                                    "clear"
                                                                </button>
                                                            }.into_any()
                                                        } else if show_empty_row {
                                                            view! {
                                                                <span class="ss-empty-row">"empty row!"</span>
                                                            }.into_any()
                                                        } else {
                                                            ().into_any()
                                                        }}
                                                    </td>
                                                </tr>
                                            }
                                        }).collect_view();

                                        view! {
                                            <div class="ss-table-scroll">
                                                <table class="ss-ability-table">
                                                    <thead>
                                                        <tr>
                                                            <th scope="col">"lvl"</th>
                                                            {headings}
                                                            <th scope="col">"clear"</th>
                                                        </tr>
                                                        <tr class="ss-optimize-row">
                                                            <th scope="row"></th>
                                                            {optimize_buttons}
                                                            <td class="ss-clear-cell">
                                                                <button
                                                                    class="pure-button ss-clear-all-button"
                                                                    type="button"
                                                                    on:click=move |_| {
                                                                        let maximum = clear_all_societies[selected_society.get()].max_level;
                                                                        selections.set(vec![None; maximum]);
                                                                    }
                                                                >
                                                                    "clear all"
                                                                </button>
                                                            </td>
                                                        </tr>
                                                    </thead>
                                                    <tbody>{rows}</tbody>
                                                </table>
                                            </div>
                                        }.into_any()
                                    }
                                    Err(error) => view! { <p class="warning">{error.to_string()}</p> }.into_any(),
                                }
                            }}
                        </Panel>
                    </div>
                </div>
            </main>
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    mount_to_body(App);
}

fn format_ability_label(key: &secret_society_core::AbilityKey) -> String {
    match (key.category.as_str(), key.target.as_str()) {
        ("Statmax", target) => target.to_owned(),
        (category, "all") => category.to_owned(),
        (category, target) => format!("{category} of {target}"),
    }
}

fn ability_columns(society: &secret_society_core::Society) -> Vec<secret_society_core::AbilityKey> {
    let mut columns = Vec::new();
    for level in &society.levels {
        for ability in &level.options {
            if !columns.contains(&ability.key) {
                columns.push(ability.key.clone());
            }
        }
    }
    columns.sort_by(compare_ability_keys);
    columns
}

fn clear_hidden_selections(selections: &mut [Option<usize>], visible_level_count: usize) {
    for selection in selections.iter_mut().skip(visible_level_count) {
        *selection = None;
    }
}

fn is_empty_before_last_selection(
    selections: &[Option<usize>],
    level_index: usize,
    visible_level_count: usize,
) -> bool {
    selections.get(level_index).is_some_and(Option::is_none)
        && selections
            .iter()
            .take(visible_level_count)
            .rposition(Option::is_some)
            .is_some_and(|last_selected| level_index < last_selected)
}

fn ability_order(key: &secret_society_core::AbilityKey) -> u8 {
    match (key.category.as_str(), key.target.as_str()) {
        ("Statmax", "str") => 0,
        ("Statmax", "dex") => 1,
        ("Statmax", "con") => 2,
        ("Statmax", "int") => 3,
        ("Statmax", "wis") => 4,
        ("Skillmax", "all") => 5,
        ("Spellmax", "all") => 6,
        _ => 7,
    }
}

fn compare_ability_keys(
    left: &secret_society_core::AbilityKey,
    right: &secret_society_core::AbilityKey,
) -> std::cmp::Ordering {
    ability_order(left)
        .cmp(&ability_order(right))
        .then_with(|| left.category.cmp(&right.category))
        .then_with(|| left.target.cmp(&right.target))
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use secret_society_core::AbilityKey;

    #[test]
    fn hiding_levels_clears_their_selections_without_changing_vector_length() {
        let mut selections = vec![Some(0), Some(2), Some(1), Some(3)];

        clear_hidden_selections(&mut selections, 2);

        assert_eq!(selections, vec![Some(0), Some(2), None, None]);
    }

    #[test]
    fn empty_row_notice_covers_blanks_before_last_visible_selection_only() {
        let selections = [None, Some(0), None, Some(1), None, Some(0)];
        let expected = [true, false, true, false, false];

        let actual = (0..5)
            .map(|level_index| is_empty_before_last_selection(&selections, level_index, 5))
            .collect::<Vec<_>>();

        assert_eq!(actual, expected);
        assert!(!is_empty_before_last_selection(&[None, None], 0, 2));
    }

    #[test]
    fn ability_columns_are_unique_and_include_later_society_levels() {
        let war = bundled_societies()
            .expect("bundled society data should parse")
            .into_iter()
            .find(|society| society.command == "war+")
            .expect("War+ should be bundled");
        let columns = ability_columns(&war);
        let unique_columns = columns.iter().collect::<HashSet<_>>();

        assert_eq!(columns.len(), unique_columns.len());
        assert!(columns.contains(&AbilityKey {
            category: "Statmax".to_owned(),
            target: "dex".to_owned(),
        }));
    }

    #[test]
    fn ability_labels_are_simplified_for_the_table() {
        assert_eq!(
            format_ability_label(&AbilityKey {
                category: "Statmax".to_owned(),
                target: "wis".to_owned(),
            }),
            "wis"
        );
        assert_eq!(
            format_ability_label(&AbilityKey {
                category: "Spellmax".to_owned(),
                target: "all".to_owned(),
            }),
            "Spellmax"
        );
        assert_eq!(
            format_ability_label(&AbilityKey {
                category: "Skillmax".to_owned(),
                target: "parry".to_owned(),
            }),
            "Skillmax of parry"
        );
    }

    #[test]
    fn ability_order_uses_the_requested_priority_then_sorts_the_rest() {
        let mut keys = [
            AbilityKey {
                category: "Skillmax".to_owned(),
                target: "parry".to_owned(),
            },
            AbilityKey {
                category: "Spellmax".to_owned(),
                target: "all".to_owned(),
            },
            AbilityKey {
                category: "Statmax".to_owned(),
                target: "wis".to_owned(),
            },
            AbilityKey {
                category: "Skillmax".to_owned(),
                target: "all".to_owned(),
            },
            AbilityKey {
                category: "Statmax".to_owned(),
                target: "str".to_owned(),
            },
            AbilityKey {
                category: "Statmax".to_owned(),
                target: "int".to_owned(),
            },
            AbilityKey {
                category: "Statmax".to_owned(),
                target: "con".to_owned(),
            },
            AbilityKey {
                category: "Statmax".to_owned(),
                target: "dex".to_owned(),
            },
        ];

        keys.sort_by(compare_ability_keys);

        assert_eq!(
            keys.iter().map(format_ability_label).collect::<Vec<_>>(),
            [
                "str",
                "dex",
                "con",
                "int",
                "wis",
                "Skillmax",
                "Spellmax",
                "Skillmax of parry",
            ]
        );
    }
}
