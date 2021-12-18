use crate::provider::theme::*;
use yew::prelude::{html, Component, Context, Html};
use yew_router::prelude::{BrowserRouter, Routable, Switch};

pub struct App;

impl Component for App {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
    <BrowserRouter>
      <ThemeProvider>
        <Switch<Route> render={Switch::render(switch)} />
      </ThemeProvider>
    </BrowserRouter>
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
  #[at("/")]
  Home,
  #[at("/about")]
  About,
  #[not_found]
  #[at("/404")]
  NotFound,
}

fn switch(routes: &Route) -> Html {
  match routes {
    // Route::Home => {
    //   html! {<div>{"Home"}</div>}
    // }
    // Route::About => {
    //   html! {<div>{"About"}</div>}
    // }
    _ => {
      // html! {<div>{"404 Not Found"}</div>}
      // html! {<crate::pages::museum::MuseumPage />}
      html! {<crate::pages::issue::Page />}
    }
  }
}
