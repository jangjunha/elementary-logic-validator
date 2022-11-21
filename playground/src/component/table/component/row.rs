use itertools::Itertools;
use web_sys::HtmlInputElement;
use yew::{
  events::{InputEvent, KeyboardEvent},
  function_component, html,
  virtual_dom::AttrValue,
  Callback, Properties, TargetCast,
};

#[derive(Properties, PartialEq)]
pub struct RowProps {
  #[prop_or(vec![])]
  pub dependents: Vec<usize>,
  pub num: usize,
  pub sentence: AttrValue,
  pub derivation: AttrValue,

  #[prop_or(Callback::noop())]
  pub on_change_sentence: Callback<String>,
  #[prop_or(Callback::noop())]
  pub on_format_sentence: Callback<()>,
  #[prop_or(Callback::noop())]
  pub on_append_row: Callback<()>,
}

#[function_component(Row)]
pub fn row(props: &RowProps) -> Html {
  let handle_input = {
    let on_change_sentence = props.on_change_sentence.clone();
    Callback::from(move |e: InputEvent| {
      let target: HtmlInputElement = e.target_unchecked_into();
      on_change_sentence.emit(target.value());
    })
  };
  let handle_sentence_keypress = {
    let on_append_row = props.on_append_row.clone();
    let on_format_sentence = props.on_format_sentence.clone();
    Callback::from(move |e: KeyboardEvent| {
      match e.key().as_str() {
        "Enter" if e.meta_key() || e.ctrl_key() => on_append_row.emit(()),
        "Enter" => on_format_sentence.emit(()),
        _ => {}
      };
    })
  };

  html! {
    <tr>
      <td>{ &props.dependents.iter().join(",") }</td>
      <td class="text-right">{ &props.num }</td>
      <td>
        <input type="text" class="w-full" value={props.sentence.clone()} oninput={handle_input} onkeypress={handle_sentence_keypress} />
      </td>
      <td>
        <input type="text" class="w-full" value={props.derivation.clone()} />
      </td>
    </tr>
  }
}
