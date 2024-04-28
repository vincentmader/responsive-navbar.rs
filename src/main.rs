use stylist::css;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let class = css!("");

    html! {
        <div {class}>
            { "home" }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
