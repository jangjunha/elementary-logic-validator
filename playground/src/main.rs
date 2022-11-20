use yew::{function_component, html};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{"1차논리 검증기"}</h1>
        </>
    }
}


fn main() {
    yew::start_app::<App>();
}
