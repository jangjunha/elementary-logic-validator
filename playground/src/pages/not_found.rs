use yew::{function_component, html};

#[function_component(NotFound)]
pub fn not_found() -> Html {
  html! {
    <>
      <h1>{"Page Not Found"}</h1>
    </>
  }
}
