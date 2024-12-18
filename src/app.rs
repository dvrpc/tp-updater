use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

const INDICATORS: &[&str] = &[
    "Air Quality",
    "Bridge Conditions",
    "Business Formations",
    "Community Integration",
    "Commute Mode",
    "Congestion",
    "Educational Attainment",
    "Greenhouse Gas Emissions",
    "Gross Domestic Product",
    "Housing Affordability",
    "Housing Permits",
    "Income",
    "Job Growth",
    "Labor Force",
    "Land Consumption",
    "Miles Driven",
    "Mortgage Lending",
    "Pavement Conditions",
    "Population Growth",
    "Transit Conditions",
    "Transit Ridership",
    "Transportation Safety",
    "Water Quality",
];

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/tp-updater.css"/>

        <Title text="Tracking Progress Updater"/>

        <Router>
            <Header/>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
        </Router>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header>
            <a href="https://sites.google.com/a/dvrpc.org/webteam/">"Intranet"</a>
            <h1>"Tracking Progress Updates"</h1>
        </header>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    // Set up server actions so both components can use/track them.
    let add_action = ServerAction::<AddIndicator>::new();
    let remove_action = ServerAction::<RemoveIndicator>::new();
    view! {
        <main>
            <div id="left"><UpdateIndicators add_action remove_action/></div>
            <div id="right"><RecentlyUpdated add_action remove_action/></div>
        </main>
    }
}

#[component]
fn UpdateIndicators(
    add_action: ServerAction<AddIndicator>,
    remove_action: ServerAction<RemoveIndicator>,
) -> impl IntoView {
    let select_start = "Select Indicator";
    let (value, set_value) = signal(select_start.to_string());

    // Create closure for setting 'disabled' on buttons if on this starting text.
    let is_disabled = move || value.get() == select_start;

    view! {
        <div>
            <p>"An overlay can be added to indicator tiles to let users know they have recently been updated with new data. The overlays will expire after 30 days."</p>
            <ul>
                <li>"Select the indicator from the dropdown and use the 'Add' button to add an overlay to it."</li>
                <li>"Use the 'Remove' button if you want to remove an indicator's overlay before the 30-day expiration."</li>
            </ul>

            <div id="update-area">
                <select on:change:target=move |ev| {
                    set_value.set(ev.target().value());
                }
                prop:value=move || value.get().to_string()>
                    <option value=value.get().to_string() selected>{value.get()}</option>
                    { INDICATORS.iter()
                        .map(|i| view! {
                            <option value=i.to_string()>{i.to_string()}</option>
                        }).collect::<Vec<_>>()
                    }
                </select>

                <button on:click=move |_| {
                    add_action.dispatch(AddIndicator {
                        ind: value.get()
                    });
                }
                prop:disabled=is_disabled
                >Add Indicator</button>

                <button on:click=move |_| {
                    remove_action.dispatch(RemoveIndicator {
                        ind: value.get()
                    });
                }
                prop:disabled=is_disabled
                >Remove Indicator</button>
            </div>
        </div>
    }
}

#[server]
pub async fn add_indicator(ind: String) -> Result<(), ServerFnError> {
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    dotenvy::dotenv().expect("Unable to load .env file");

    let database = env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database)
        .await?;

    match sqlx::query!("insert into updates (indicator) VALUES ($1)", ind)
        .execute(&pool)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server]
pub async fn remove_indicator(ind: String) -> Result<(), ServerFnError> {
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    dotenvy::dotenv().expect("Unable to load .env file");

    let database = env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database)
        .await?;

    match sqlx::query!("delete from updates where indicator = $1", ind)
        .execute(&pool)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[component]
fn RecentlyUpdated(
    add_action: ServerAction<AddIndicator>,
    remove_action: ServerAction<RemoveIndicator>,
) -> impl IntoView {
    // This is mostly taken from example at
    // <https://github.com/leptos-rs/leptos/blob/main/examples/server_fns_axum/src/app.rs>

    // this resource will hold the Vec<String> of indicators
    // passing it action.version() means it will refetch whenever the action resolves successfully
    let indicators = Resource::new(
        move || (add_action.version().get(), remove_action.version().get()),
        |_| get_indicators(),
    );

    view! {
        <h2>Recently Updated Indicators</h2>
        <ul>
            // put the indicators into a list while handling any ServerFn errors.
            <Transition fallback=move || view! {""}>
                <ErrorBoundary fallback=|_| { view! { "Unable to fetch indicators." } }>
                    { move || {
                        indicators.get().map(|v| {
                            match v {
                                Ok(v) => {
                                    if v.is_empty() {
                                        Ok(view! { "No indicators added." }.into_any())
                                    } else {
                                        Ok(v.into_iter().map(|ind| {
                                            view! {
                                                 <li>{ind}</li>
                                            }
                                        }).collect::<Vec<_>>().into_any())
                                    }
                                }
                                Err(e) => Err(e)
                            }
                        })
                    }}
                </ErrorBoundary>
            </Transition>
         </ul>
    }
}

#[server]
pub async fn get_indicators() -> Result<Vec<String>, ServerFnError> {
    use chrono::{Local, TimeDelta};
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    dotenvy::dotenv().expect("Unable to load .env file");

    let database = env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database)
        .await?;

    let one_month_ago = Local::now().date_naive() - TimeDelta::days(30);
    let indicators = sqlx::query!(
        "SELECT indicator FROM updates WHERE updated >= $1",
        one_month_ago
    )
    .fetch_all(&pool)
    .await?;

    let indicators = indicators
        .into_iter()
        .map(|v| v.indicator)
        .collect::<Vec<String>>();

    Ok(indicators)
}
