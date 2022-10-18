use std::collections::HashMap;

use dotenvy_macro::dotenv;
use gloo_console::log;
use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use yew::prelude::*;

const API_BASE: &str = dotenv!("API_BASE");

enum Msg {
    Add,
    Remove,
    Status(String),
    Select(String),
    SetCurrentIndicators(Vec<String>),
}

/// Fetch recently updated indicators from API
async fn get_updated_indicators() -> Result<Vec<String>, String> {
    let r = Request::get(&format!("{API_BASE}/indicators")).send().await;

    match r {
        Ok(response) => {
            let json: Result<Vec<String>, _> = response.json().await;
            match json {
                Ok(mut v) => {
                    v.sort();
                    Ok(v)
                }
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

/// Add new indicator
async fn add_indicator(indicator: String) -> Result<String, String> {
    let payload = HashMap::from([("name", indicator)]);

    let r = Request::post(&format!("{API_BASE}/indicators"))
        .json(&payload)
        .unwrap()
        .send()
        .await;

    match r {
        Ok(_) => Ok("ok".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

/// Delete indicator
async fn remove_indicator(indicator: String) -> Result<String, String> {
    let payload = HashMap::from([("name", indicator)]);

    let r = Request::delete(&format!("{API_BASE}/indicators"))
        .json(&payload)
        .unwrap()
        .send()
        .await;

    match r {
        Ok(_) => Ok("ok".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

struct Model {
    indicator: Option<String>,
    updated_indicators: Vec<String>,
    update_status: Option<String>,
}

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

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Model {
            indicator: None,
            updated_indicators: vec![],
            update_status: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let onchange = link.callback(|e: Event| {
            Msg::Select(e.target_unchecked_into::<HtmlInputElement>().value())
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
                        if let Some(v) = &self.update_status {
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
        match msg {
            Msg::Add => match self.indicator.clone() {
                None => (),
                Some(v) => ctx.link().send_future(async move {
                    match add_indicator(v).await {
                        Ok(_) => Msg::Status("Indicator added".to_string()),
                        Err(e) => Msg::Status(e.to_string()),
                    }
                }),
            },
            Msg::Remove => match self.indicator.clone() {
                None => (),
                Some(v) => {
                    ctx.link().send_future(async move {
                        match remove_indicator(v).await {
                            Ok(_) => Msg::Status("Indicator removed".to_string()),
                            Err(e) => Msg::Status(e.to_string()),
                        }
                    });
                }
            },
            Msg::Status(status) => {
                self.update_status = Some(status);
                ctx.link().send_future(async {
                    match get_updated_indicators().await {
                        Ok(v) => Msg::SetCurrentIndicators(v),
                        Err(e) => Msg::SetCurrentIndicators(vec![e]),
                    }
                })
            }
            Msg::Select(ind) => {
                if !INDICATORS.contains(&ind.as_str()) {
                    self.indicator = None;
                } else {
                    self.indicator = Some(ind);
                }
            }
            Msg::SetCurrentIndicators(v) => {
                self.updated_indicators = v;
            }
        };
        true
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_future(async {
                match get_updated_indicators().await {
                    Ok(v) => Msg::SetCurrentIndicators(v),
                    Err(e) => Msg::SetCurrentIndicators(vec![e]),
                }
            });
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
