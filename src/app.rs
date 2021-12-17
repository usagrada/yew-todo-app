use crate::provider::theme::*;
use yew::prelude::{html, Callback, Component, Context, Html};
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
        <Hello />
        <Switch<Route> render={Switch::render(switch)} />
      </ThemeProvider>
    </BrowserRouter>
    }
  }
}

struct Hello;

impl Component for Hello {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let (theme, _) = ctx
      .link()
      .context::<Theme>(Callback::noop())
      .expect("theme to be set");
    html! {
      <div>{theme.foreground} </div>
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
      html! {<crate::pages::museum::MuseumPage />}
    }
  }
}
