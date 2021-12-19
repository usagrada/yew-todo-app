use yew::{
  classes, function_component, html, use_context, Callback, Children, Component, Context,
  ContextProvider, Html, Properties,
};

#[derive(Clone)]
pub struct Provider {
  value: usize,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
  pub children: Children,
}

pub enum ProviderMessage {
  Msg,
}

type MyClick = yew::Callback<yew::MouseEvent>;

impl Component for Provider {
  type Message = ProviderMessage;
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self { value: 0 }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      ProviderMessage::Msg => {
        log::info!("click");
        self.value += 1;
      }
    }
    true
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let onclick: MyClick = ctx.link().callback(|_| ProviderMessage::Msg);

    html! {
      <ContextProvider<MyClick> context={&onclick}>
        <ContextProvider<usize> context={self.value}>
            { ctx.props().children.clone()}
        </ContextProvider<usize>>
      </ContextProvider<MyClick>>
    }
  }
}

pub struct Page;

impl Component for Page {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <div class={classes!("museum__container")}>
      <Provider>
        <div>{"struct component"}</div>
        <View />
        <div>{"functional component"}</div>
        <ViewFunc />
      </Provider>
      </div>
    }
  }
}

pub struct View;

impl Component for View {
  type Message = ProviderMessage;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let (click, _) = ctx
      .link()
      .context::<MyClick>(Callback::noop())
      .expect("field to be set");
    let (value, _) = ctx
      .link()
      .context::<usize>(Callback::noop())
      .expect("field to be set");

    html! {
      <div onclick={click}>
        {value}
        <button>{"click"}</button>
      </div>
    }
  }
}

/// This component has access to the context
#[function_component(ViewFunc)]
pub fn view() -> Html {
  let value = use_context::<usize>().expect("no ctx found");
  let click = use_context::<MyClick>().expect("no ctx found");

  html! {
    <div onclick={click}>
      {value}
      <button>{"click"}</button>
    </div>
  }
}
