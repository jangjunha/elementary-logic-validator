mod pages;

use self::pages::Home;
use yew::{function_component, html, Html};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum HelpRoute {
  #[at("/help/")]
  Home,
}

fn switch(routes: HelpRoute) -> Html {
  match routes {
    HelpRoute::Home => html! {
      <Home />
    },
  }
}

#[function_component(Help)]
pub fn help() -> Html {
  html! {
      <Switch<HelpRoute> render={switch} />
  }
}
