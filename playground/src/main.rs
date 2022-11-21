mod component;
mod hooks;

use yew::{function_component, html};

use component::table::Table;

#[function_component(App)]
fn app() -> Html {
  html! {
    <>
      <h1>{"1차논리 검증기"}</h1>
      <section class="max-w-3xl mx-auto">
        <Table />
      </section>
      <div>
        <p>{"Ctrl + Enter : 아래에 행 삽입"}</p>
        <p>{"Enter: Format"}</p>
      </div>
    </>
  }
}

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  yew::start_app::<App>();
}
