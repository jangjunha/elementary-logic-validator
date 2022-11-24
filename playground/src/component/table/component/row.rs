use std::collections::HashSet;

use super::super::parser::{parse_exp, parse_rule};
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
  #[prop_or(Callback::noop())]
  pub on_focus: Callback<()>,
  #[prop_or(Callback::noop())]
  pub on_blur: Callback<()>,
}

#[function_component(Row)]
pub fn row(props: &RowProps) -> Html {
  let is_sentence_syntax_valid = use_memo(
    |sentence| match parse_exp(sentence) {
      Ok(_) => true,
      Err(()) => false,
    },
    props.sentence.clone(),
  );
  let is_rule_syntax_valid = use_memo(
    |rule| match parse_rule(rule) {
      Ok(_) => true,
      Err(()) => false,
    },
    props.derivation.clone(),
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

  const CLS_CELL: &'static str = "h-full p-[10px] pb-[9px] border-b border-b-gray-300";
  let cls_input = classes!(
    "w-full",
    "focus:outline-none",
    "focus:border-2",
    "focus:px-[8px]",
    "focus:py-[7px]",
    props.readonly.then_some(classes!("pt-[]", "pb-[]")).unwrap_or(classes!(
      "pt-[8px]",
      "pb-[6px]",
      "border-b-2",
      "bg-gray-100"
    )),
  );
  const CLS_INPUT_VALID: &'static str = "focus:border-green-400";
  const CLS_INPUT_INVALID: &'static str = "border-b-red-400 focus:border-red-400";
  const CLS_DIV_VALID: &'static str = "";
  const CLS_DIV_INVALID: &'static str = "border-b-red-400";
  const CLS_SYNTAX_VALID: &'static str = "";
  const CLS_SYNTAX_INVALID: &'static str = "focus:underline focus:decoration-wavy focus:decoration-red-400";
  html! {
    <tr class={classes!("h-fit", "[&>td]:h-full", props.class.to_string())}>
      <td class={classes!("text-gray-400", "break-word")}>
        <div class={classes!(
          CLS_CELL,
          props.is_dependents_complete
            .then_some(CLS_DIV_VALID.clone())
            .unwrap_or(CLS_DIV_INVALID.clone()),
        )}>
          { &props.dependents.iter().sorted_unstable().join(",") }
        </div>
      </td>
      <td class={classes!("text-gray-400")}>
        <div class={classes!(CLS_CELL, "text-right")}>{ &props.num }</div>
      </td>
      <td>
        <input
          type="text"
          class={classes!(
            CLS_CELL,
            cls_input.clone(),
            is_sentence_syntax_valid
              .then_some(CLS_SYNTAX_VALID)
              .unwrap_or(CLS_SYNTAX_INVALID),
            is_sentence_syntax_valid
              .then_some(CLS_INPUT_VALID)
              .unwrap_or(CLS_INPUT_INVALID),
          )}
          value={props.sentence.clone()}
          readonly={props.readonly}
          oninput={handle_sentence_input}
          onkeypress={handle_inputs_keypress.clone()}
          onfocus={props.on_focus.reform(|_| ())}
          onblur={props.on_blur.reform(|_| ())}
        />
      </td>
      <td>
        <input
          type="text"
          class={classes!(
            CLS_CELL,
            cls_input.clone(),
            is_rule_syntax_valid
              .then_some(CLS_SYNTAX_VALID)
              .unwrap_or(CLS_SYNTAX_INVALID),
            (*is_rule_syntax_valid && props.is_derivation_valid)
              .then_some(CLS_INPUT_VALID)
              .unwrap_or(CLS_INPUT_INVALID),
          )}
          value={props.derivation.clone()}
          readonly={props.readonly}
          oninput={handle_derivation_input}
          onkeypress={handle_inputs_keypress}
          onfocus={props.on_focus.reform(|_| ())}
          onblur={props.on_blur.reform(|_| ())}
        />
      </td>
    </tr>
  }
}
