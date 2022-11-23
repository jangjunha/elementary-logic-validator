mod component;
mod hooks;
mod pages;

use self::pages::{Help, Home, NotFound};
use yew::{function_component, html, Html};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  Home,
  #[at("/help/")]
  HelpHome,
  #[at("/help/:s")]
  Help,
  #[not_found]
  #[at("/404/")]
  NotFound,
}

fn switch(routes: &Route) -> Html {
  match routes {
    Route::Home => html! {
      <Home />
    },
    Route::HelpHome | Route::Help => html! {
      <Help />
    },
    Route::NotFound => html! {
      <NotFound />
    },
  }
}

#[function_component(App)]
fn app() -> Html {
  html! {
    <div class="prose max-w-2xl mx-auto my-16">
      <h1>{"1차논리 검증기"}</h1>

      <BrowserRouter>
        <Switch<Route> render={Switch::render(switch)} />
      </BrowserRouter>

      <footer class="my-16 border-t text-center">
        <p>
          <a href="/">{"검증기"}</a>{" "}
          <a href="/help/">{"정보 및 도움말"}</a>
        </p>
      </footer>
    </div>
  }
}

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  yew::start_app::<App>();
}
