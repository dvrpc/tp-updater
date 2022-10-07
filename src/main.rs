use yew::prelude::*;

struct Model {}

enum Msg {}

enum Indicators {
    AirQuality,
    BridgeConditions,
    CommuteMode,
    Congestion,
    EducationalAttainment,
    ExportedGoods,
    GlobalConnectivity,
    GreenhouseGasEmissions,
    HousingActivity,
    HousingAffordability,
    IncomeDisparities,
    Innovataion,
    JobGrowth,
    LandConsumption,
    MilesDriven,
    PavementConditions,
    PopulationGrowth,
    RacialAndEthnicDisparies,
    RoadwaySafety,
    SexDisparities,
    TransitConditions,
    TransitRidership,
    WaterQuality,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Model {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <header>
                    <a href="https://sites.google.com/a/dvrpc.org/webteam/">{ "Intranet" }</a>
                </header>
                <main>
                    <div id="left">
                        <h1>{ "Tracking Progress Updates" }</h1>
                        <p>{ "Add an overlay to indicator tiles to let users know they have recently been updated with new data." }</p>
                        <ul>
                            <li>{ " Use the 'Add' form to add an overlay for a newly updated indicator. These overlays expire 30 days after their creation."}</li>
                            <li>{ " Use the 'Remove' form if you want to remove an indicator overlay before the 30 day expiration date. "}</li>
                        </ul>
                    </div>
                    <div id="right">
                        <h1>{ "Updated Indicators" }</h1>
                        <p>{ "[Display of current updated indicators here (or message saying none)]"}</p>
                        <hr />
                        <div id="buttons">
                            <button class="left">{ "Add Indicator ➠" }</button>
                            <select class="right" name="add"></select>
                            <button class="left">{ "Remove Indicator ➠" }</button>
                            <select class="right" name="remove"></select>
                        </div>
                    </div>
                </main>
            </>
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
}

fn main() {
    yew::start_app::<Model>();
}
