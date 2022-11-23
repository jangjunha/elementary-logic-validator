use std::collections::HashSet;

use super::super::parser::parse_exp;
use crate::hooks::use_memo;
use itertools::Itertools;
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
  #[prop_or(AttrValue::from(""))]
  pub class: AttrValue,
  #[prop_or(false)]
  pub readonly: bool,

  #[prop_or(HashSet::from([]))]
  pub dependents: HashSet<usize>,
  #[prop_or(false)]
  pub is_dependents_complete: bool,
  pub num: usize,
  pub sentence: AttrValue,
  pub derivation: AttrValue,
  #[prop_or(false)]
  pub is_derivation_valid: bool,

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
      Ok(_) => true,
      Err(()) => false,
    },
    props.sentence.clone(),
  );

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
    <tr class={classes!(props.class.to_string())}>
      <td class={classes!(props.is_dependents_complete.then_some("").unwrap_or("bg-red-200"))}>
        { &props.dependents.iter().sorted_unstable().join(",") }
      </td>
      <td class="text-right">{ &props.num }</td>
      <td>
        <input
          type="text"
          class={classes!(
            "w-full",
            "bg-transparent",
            "border-b",
            is_sentence_valid
              .then_some("border-b-transparent")
              .unwrap_or("border-b-red-400 focus:border-b-transparent focus:outline focus:outline-2 focus:outline-red-400"),
          )}
          value={props.sentence.clone()}
          readonly={props.readonly}
          oninput={handle_sentence_input}
          onkeypress={handle_inputs_keypress.clone()}
        />
      </td>
      <td class={classes!(props.is_derivation_valid.then_some("bg-green-100").unwrap_or("bg-red-200"))}>
        <input
          type="text"
          class="w-full bg-transparent"
          value={props.derivation.clone()}
          readonly={props.readonly}
          oninput={handle_derivation_input}
          onkeypress={handle_inputs_keypress}
        />
      </td>
    </tr>
  }
}
