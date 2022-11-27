mod component;
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

fn switch(routes: Route) -> Html {
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
      <div class="prose prose-a:text-blue-500 max-w-2xl mx-auto my-16">
        <header class="flex items-baseline justify-between">
          <h1 class="leading-none">{"1차논리 검증기"}</h1>
          <nav>
            <ul class="list-none p-0 flex gap-4">
              <li><Link<Route> to={Route::Home}>{"검증기"}</Link<Route>>{" "}</li>
              <li><Link<Route> to={Route::HelpHome}>{"정보 및 도움말"}</Link<Route>></li>
            </ul>
          </nav>
        </header>

        <Switch<Route> render={switch} />
      </div>
    </BrowserRouter>
  }
}

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  yew::Renderer::<App>::new().render();
}
