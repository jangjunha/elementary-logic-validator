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
    <BrowserRouter>
      <div class="prose max-w-2xl mx-auto my-16">
        <h1>{"1차논리 검증기"}</h1>

        <Switch<Route> render={Switch::render(switch)} />

        <footer class="my-16 border-t text-center">
          <p>
            <Link<Route> to={Route::Home}>{"검증기"}</Link<Route>>{" "}
            <Link<Route> to={Route::HelpHome}>{"정보 및 도움말"}</Link<Route>>
          </p>
        </footer>
      </div>
    </BrowserRouter>
  }
}

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  yew::start_app::<App>();
}
