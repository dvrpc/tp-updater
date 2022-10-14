use gloo_console::log;
use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use yew::prelude::*;

const API_BASE: &str = "http://localhost:8000/tracking-progress/v1";

/// Fetch recently updated indicators from API
async fn get_updated_indicators() -> Result<Vec<String>, String> {
    let fetched_ur = Request::get(&format!("{API_BASE}/indicators")).send().await;

    match fetched_ur {
        Ok(response) => {
            let json: Result<Vec<String>, _> = response.json().await;
            match json {
                Ok(v) => Ok(v),
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
enum Msg {
    Add,
    Remove,
    Select(String),
    SetCurrentIndicators(Vec<String>),
}

struct Model {
    indicator: Option<String>,
    updated_indicators: Vec<String>,
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
    "Innovataion",
    "Job Growth",
    "Land Consumption",
    "Miles Driven",
    "Pavement Conditions",
    "Population  Growth",
    "Racial and Ethnic Disparies",
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
            html! { <p>{ format!("{:#?}", self.updated_indicators) }</p> }
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
                </header>
                <main>
                    <div id="left">
                        <h1>{ "Tracking Progress Updates" }</h1>
                        <p>{ "An overlay can be added to indicator tiles to let users know they have recently been updated with new data. The overlays will expire after 30 days." }
                            <ul>
                                <li>{ " Select the indicator from the dropdown and use the 'Add' button to add an overlay to it."}</li>
                                <li>{ " Use the 'Remove' button if you want to remove an indicator's overlay before the 30-day expiration. "}</li>
                            </ul>
                        </p>
                    </div>
                    <div id="right">
                        <h1>{ "Updated Indicators" }</h1>

                        <p class="center">{ updated_indicators }</p>
                        <hr />
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
                    </div>
                </main>
            </>
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add => match self.indicator {
                None => log!(format!("Please select an indicator to add.")),
                Some(_) => log!(format!("{:?}", self.indicator)),
            },
            Msg::Remove => match self.indicator {
                None => log!(format!("Please select an indicator to remove.")),
                Some(_) => log!(format!("{:?}", self.indicator)),
            },
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
