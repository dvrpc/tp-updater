use std::collections::HashMap;

use dotenvy_macro::dotenv;
use gloo_console::log;
use gloo_dialogs::alert;
use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use yew::prelude::*;

const API_BASE: &str = dotenv!("API_BASE");

const INDICATORS: &[&str] = &[
    "Air Quality",
    "Bridge Conditions",
    "Commute Mode",
    "Congestion",
    "Educational Attainment",
    "Exported Goods",
    "Global Connectivity",
    "Greenhouse Gas Emissions",
    "Housing Activity",
    "Housing Affordability",
    "Income Disparities",
    "Innovation",
    "Job Growth",
    "Land Consumption",
    "Miles Driven",
    "Pavement Conditions",
    "Population Growth",
    "Racial and Ethnic Disparities",
    "Roadway Safety",
    "Sex Disparities",
    "Transit Conditions",
    "Transit Ridership",
    "Water Quality",
];

enum Msg {
    SelectIndicator(String),
    Add,
    Remove,
    Confirm(String),
    GetUpdatedIndicators,
    SetUpdatedIndicators(Vec<String>),
    Error(String),
}

struct App {
    selected_indicator: Option<String>,
    updated_indicators: Vec<String>,
    confirmation_message: Option<String>,
    error: Option<String>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {
            selected_indicator: None,
            updated_indicators: vec![],
            confirmation_message: None,
            error: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(v) = &self.error {
            alert(v);
        }

        let link = ctx.link();

        let onchange = link.callback(|e: Event| {
            Msg::SelectIndicator(e.target_unchecked_into::<HtmlInputElement>().value())
        });

        let updated_indicators = if self.updated_indicators.is_empty() {
            html! { <p>{ "There are no recently updated indicators."}</p> }
        } else {
            html! { <ul> {
                    self.updated_indicators
                        .iter()
                        .map(|ind| html! { <li>{format!("{}", ind)}</li> })
                        .collect::<Html>()
                    }
            </ul>}
        };

        let indicators = INDICATORS
            .iter()
            .map(|ind| {
                html! { <option>{ ind }</option> }
            })
            .collect::<Html>();

        html! {
            <>
                <header>
                    <a href="https://sites.google.com/a/dvrpc.org/webteam/">{ "Intranet" }</a>
                    <h1>{ "Tracking Progress Updates" }</h1>
                </header>
                <main>
                    <div id="left">
                        <p>{ "An overlay can be added to indicator tiles to let users know they have recently been updated with new data. The overlays will expire after 30 days." }
                            <ul>
                                <li>{ " Select the indicator from the dropdown and use the 'Add' button to add an overlay to it."}</li>
                                <li>{ " Use the 'Remove' button if you want to remove an indicator's overlay before the 30-day expiration. "}</li>
                            </ul>
                        </p>
                        <div id="update-area">
                            <select {onchange}>
                                <option SELECTED="true">{"Select Indicator"}</option>
                                { indicators }
                            </select>
                            <button onclick={link.callback(|_| Msg::Add)}>
                                { "Add It" }
                            </button>
                            <button onclick={link.callback(|_| Msg::Remove)}>
                                { "Remove It" }
                            </button>
                        </div>
                        if let Some(v) = &self.confirmation_message {
                            <p>{ format!("{}", v) }</p>
                        }
                    </div>
                    <div id="right">
                        <div>
                            <h2>{ "Recently Updated Indicators" }</h2>
                            { updated_indicators }
                        </div>
                    </div>
                </main>
            </>
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();

        // clear any previous errors
        self.error = None;

        match msg {
            Msg::SelectIndicator(ind) => {
                if !INDICATORS.contains(&ind.as_str()) {
                    self.selected_indicator = None;
                } else {
                    self.selected_indicator = Some(ind);
                }
            }
            Msg::Add => match self.selected_indicator.clone() {
                None => link.send_message(Msg::Error("No indicator selected".to_string())),
                Some(v) => link.send_future(async move {
                    match Request::post(&format!("{API_BASE}/indicators"))
                        .json(&HashMap::from([("name", v)]))
                        .unwrap()
                        .send()
                        .await
                    {
                        Ok(r) => match r.status() {
                            201 => Msg::Confirm("Indicator added".to_string()),
                            500 => {
                                Msg::Error("Error with API, please try again later.".to_string())
                            }
                            _ => Msg::Error("Undefined response from API".to_string()),
                        },

                        Err(e) => Msg::Error(e.to_string()),
                    }
                }),
            },
            Msg::Remove => match self.selected_indicator.clone() {
                None => link.send_message(Msg::Error("No indicator selected".to_string())),
                Some(v) => link.send_future(async move {
                    match Request::delete(&format!("{API_BASE}/indicators"))
                        .json(&HashMap::from([("name", &v)]))
                        .unwrap()
                        .send()
                        .await
                    {
                        Ok(r) => match r.status() {
                            200 => Msg::Confirm("Indicator removed".to_string()),
                            404 => Msg::Error(format!(
                                "Cannot remove {v}: not a recently updated indicator."
                            )),
                            500 => {
                                Msg::Error("Error with API, please try again later.".to_string())
                            }
                            _ => Msg::Error("Undefined response from API".to_string()),
                        },
                        Err(e) => Msg::Error(e.to_string()),
                    }
                }),
            },
            Msg::Confirm(message) => {
                self.confirmation_message = Some(message);
                link.send_message(Msg::GetUpdatedIndicators)
            }
            Msg::GetUpdatedIndicators => {
                link.send_future(async {
                    match Request::get(&format!("{API_BASE}/indicators")).send().await {
                        Ok(r) => match r.status() {
                            200 => {
                                let json: Result<Vec<String>, _> = r.json().await;
                                match json {
                                    Ok(mut v) => {
                                        v.sort();
                                        Msg::SetUpdatedIndicators(v)
                                    }
                                    Err(e) => Msg::Error(e.to_string()),
                                }
                            }
                            500 => Msg::Error("Error with API, please try again later".to_string()),
                            _ => Msg::Error("Undefined response from API".to_string()),
                        },
                        Err(e) => Msg::Error(e.to_string()),
                    }
                });
            }
            Msg::SetUpdatedIndicators(v) => {
                self.updated_indicators = v;
            }
            Msg::Error(error) => {
                self.error = Some(error);
            }
        };
        true
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::GetUpdatedIndicators)
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
