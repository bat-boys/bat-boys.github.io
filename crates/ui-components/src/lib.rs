use leptos::prelude::*;

/// Standard page heading, matching the site's shared stylesheet.
#[component]
pub fn PageHeader(title: &'static str) -> impl IntoView {
    view! {
        <div class="header">
            <h1>{title}</h1>
        </div>
    }
}

/// A titled section using the site's existing panel styling.
#[component]
pub fn Panel(#[prop(optional)] title: Option<String>, children: Children) -> impl IntoView {
    view! {
        <section class="equipment ss-panel">
            {title.map(|title| view! {
                <div class="box">
                    <h3>{title}</h3>
                </div>
            })}
            <div class="box ss-panel-content">{children()}</div>
        </section>
    }
}

/// A reusable summary line for a named numeric total.
#[component]
pub fn SummaryRow(label: String, value: u32) -> impl IntoView {
    view! {
        <div class="ss-summary-row">
            <span>{label}</span>
            <strong>{format!("+{value}")}</strong>
        </div>
    }
}
