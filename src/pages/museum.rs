use yew::{
  classes, function_component, html, use_context, Callback, Children, Component, Context,
  ContextProvider, Html, Properties,
};

#[derive(Clone, PartialEq)]
pub struct Field {
  height: usize,
  width: usize,
  state: Vec<Vec<usize>>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
  width: usize,
  height: usize,
  pub children: Children,
}

#[derive(Clone)]
pub struct MuseumProvider {
  field: Field,
}

pub enum MuseumProviderMessage {
  Msg,
}

type MyClick = yew::Callback<yew::MouseEvent>;

impl Component for MuseumProvider {
  type Message = MuseumProviderMessage;
  type Properties = Props;

  fn create(ctx: &Context<Self>) -> Self {
    let height = ctx.props().height;
    let width = ctx.props().width;
    // let height = 3;
    // let width = 3;
    Self {
      field: Field {
        height,
        width,
        state: vec![vec![0; width]; height],
      },
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      MuseumProviderMessage::Msg => {
        log::info!("click");
        self.field.state[1][2] += 1;
        self.field.height += 1;
        self.field.state.push(vec![0; self.field.width]);
        log::info!(
          "state: {:?}, {}",
          self.field.state.len(),
          self.field.state.len()
        );
        log::info!("height: {}", self.field.height);
      }
    }
    // false
    true
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let onclick: MyClick = ctx.link().callback(|_| MuseumProviderMessage::Msg);

    html! {
      <ContextProvider<MyClick> context={&onclick}>
        <ContextProvider<Field> context={self.field.clone()}>
            <div>{self.field.height}</div>
            <div>{self.field.width}</div>
            <div>{format!("state: {}",self.field.state[1][2])}</div>
            { ctx.props().children.clone()}
        </ContextProvider<Field>>
      </ContextProvider<MyClick>>
    }
  }
}

pub struct MuseumPage;

impl Component for MuseumPage {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    let width = 3;
    let height = 3;

    html! {
      <div class={classes!("museum__container")}>
      <MuseumProvider width={width} height={height}>
        <div>{"class component"}</div>
        <FieldView />
        <div>{"functional component"}</div>
        <FieldViewFunc />
      </MuseumProvider>
      </div>
    }
  }
}

pub struct FieldView;

impl Component for FieldView {
  type Message = MuseumProviderMessage;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      MuseumProviderMessage::Msg => {
        log::info!("click in field view");
      }
    }
    true
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let (click, _) = ctx
      .link()
      .context::<MyClick>(Callback::noop())
      .expect("field to be set");
    let (field, _) = ctx
      .link()
      .context::<Field>(Callback::noop())
      // .context::<Field>(update)
      .expect("field to be set");

    let table = (0..field.height)
      .into_iter()
      .map(|i| {
        let rows = (0..field.width)
          .into_iter()
          .map(|j| {
            let update = ctx.link().callback(|_| MuseumProviderMessage::Msg);
            html! {
              <td
                onclick={update}
                onclick={&click}
              >
                {field.state[i][j]}
              </td>
            }
          })
          .collect::<Vec<Html>>();
        html! {
          <tr>{rows}</tr>
        }
      })
      .collect::<Vec<Html>>();

    html! {
      <table class={classes!("museum__table")}>{table}</table>
    }
  }
}

/// This component has access to the context
#[function_component(FieldViewFunc)]
pub fn field() -> Html {
  let field = use_context::<Field>().expect("no ctx found");
  let click = use_context::<MyClick>().expect("no ctx found");

  let table = (0..field.height)
    .into_iter()
    .map(|i| {
      let rows = (0..field.width)
        .into_iter()
        .map(|j| {
          html! {
            <td
              onclick={&click}
            >
              {field.state[i][j]}
            </td>
          }
        })
        .collect::<Vec<Html>>();
      html! {
        <tr>{rows}</tr>
      }
    })
    .collect::<Vec<Html>>();

  html! {
    <table class={classes!("museum__table")}>{table}</table>
  }
}
