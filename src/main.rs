use gloo_console::log;
use web_sys::HtmlInputElement;
use yew::prelude::*;

enum Msg {
    Add,
    Remove,
    Select(String),
}

struct Model {
    indicator: Option<String>,
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
        Model { indicator: None }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let onchange = link.callback(|e: Event| {
            Msg::Select(e.target_unchecked_into::<HtmlInputElement>().value())
        });

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
                        <p>{ "[Display of current updated indicators here (or message saying none)]"}</p>
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
        };
        true
    }
}

fn main() {
    yew::start_app::<Model>();
}
