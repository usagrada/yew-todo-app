use yew::{html, Children, Component, Context, ContextProvider, Html, Properties};

#[derive(Clone, PartialEq)]
pub struct Theme {
  pub foreground: String,
  pub background: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
  pub children: Children,
}

#[derive(Clone)]
pub struct ThemeProvider {
  theme: Theme
}

impl Component for ThemeProvider {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {
      theme: Theme {
        foreground: "yellow".to_owned(),
        background: "pink".to_owned(),
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
        <ContextProvider<Theme> context={self.theme.clone()}>
          { ctx.props().children.clone()}
        </ContextProvider<Theme>>
    }
  }
}
