use yew::{function_component, html};

use crate::component::table::Table;

#[function_component(Home)]
pub fn home() -> Html {
  html! {
    <>
      <section>
        <Table />
      </section>
      <section class="bg-slate-100 px-8 py-1 rounded-2xl">
        <h4>{"단축키"}</h4>
        <ul>
          <li>{"Ctrl + Enter : 아래에 행 삽입"}</li>
          <li>{"Enter: Format all"}</li>
        </ul>
      </section>
    </>
  }
}
