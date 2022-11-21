use crate::hooks::use_memo;
use itertools::Itertools;
use language::parser::expression::exp as parse_exp;
use language_derivation_rule::parser::rule::rule as parse_rule;
use web_sys::HtmlInputElement;
use yew::{
  classes,
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
  pub on_change_derivation: Callback<String>,
  #[prop_or(Callback::noop())]
  pub on_format: Callback<()>,
  #[prop_or(Callback::noop())]
  pub on_append_row: Callback<()>,
}

#[function_component(Row)]
pub fn row(props: &RowProps) -> Html {
  let is_sentence_valid = use_memo(
    |sentence| match parse_exp(sentence.trim()) {
      Ok(("", _)) => true,
      _ => false,
    },
    props.sentence.clone(),
  );
  let is_derivation_valid = use_memo(
    |rule| match parse_rule(rule.trim()) {
      Ok(("", _)) => true,
      _ => false,
    },
    props.derivation.clone(),
  ); // TODO: row refs도 바른지 검사할 필요

  let handle_sentence_input = {
    let on_change_sentence = props.on_change_sentence.clone();
    Callback::from(move |e: InputEvent| {
      let target: HtmlInputElement = e.target_unchecked_into();
      on_change_sentence.emit(target.value());
    })
  };
  let handle_derivation_input = {
    let on_change_derivation = props.on_change_derivation.clone();
    Callback::from(move |e: InputEvent| {
      let target: HtmlInputElement = e.target_unchecked_into();
      on_change_derivation.emit(target.value());
    })
  };
  let handle_inputs_keypress = {
    let on_append_row = props.on_append_row.clone();
    let on_format = props.on_format.clone();
    Callback::from(move |e: KeyboardEvent| {
      match e.key().as_str() {
        "Enter" if e.meta_key() || e.ctrl_key() => on_append_row.emit(()),
        "Enter" => on_format.emit(()),
        _ => {}
      };
    })
  };

  html! {
    <tr>
      <td>{ &props.dependents.iter().join(",") }</td>
      <td class="text-right">{ &props.num }</td>
      <td class={classes!(is_sentence_valid.then_some("bg-green-200"))}>
        <input
          type="text"
          class="w-full bg-transparent"
          value={props.sentence.clone()}
          oninput={handle_sentence_input}
          onkeypress={handle_inputs_keypress.clone()}
        />
      </td>
      <td class={classes!(is_derivation_valid.then_some("bg-green-200"))}>
        <input
          type="text"
          class="w-full bg-transparent"
          value={props.derivation.clone()}
          oninput={handle_derivation_input}
          onkeypress={handle_inputs_keypress}
        />
      </td>
    </tr>
  }
}
