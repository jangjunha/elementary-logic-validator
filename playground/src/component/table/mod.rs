mod component;
mod parser;
mod state;

use std::collections::HashSet;

use itertools::izip;
use yew::{classes, function_component, html, html_nested, use_reducer, virtual_dom::AttrValue, Callback, Properties};

pub use self::state::Row;
use self::state::{Action, State};

#[derive(Properties, PartialEq)]
pub struct TableProps {
  #[prop_or(None)]
  pub default_value: Option<Vec<Row>>,
  #[prop_or(false)]
  pub readonly: bool,
}

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
  let state = use_reducer(|| match &props.default_value {
    Some(rows) => State::init_from(rows.clone()),
    None => State::init(),
  });

  let handle_format = {
    let state = state.clone();
    Callback::from(move |()| state.dispatch(Action::Format))
  };

  let consequent_deps = state
    .deps_list
    .last()
    .map(|dep| dep.nums.clone())
    .unwrap_or_else(HashSet::new);
  let length = state.rows.len();
  html! {
    <table class="table-fixed w-full font-mono">
      <thead>
        <tr>
          <th class="w-24">{"전제번호"}</th>
          <th class="w-8">{"#"}</th>
          <th class="">{"식"}</th>
          <th class="w-36">{"도출규칙"}</th>
        </tr>
      </thead>
      <tbody>
        { for izip!(state.rows.iter(), state.deps_list.iter(), state.rule_vaildity_list.iter()).enumerate().map(|(idx, (row, dep, is_rule_valid))| {
          let num = idx + 1;
          let is_last = num == length;
          let handle_change_sentence = {
            let state = state.clone();
            Callback::from(move |sentence: String| {
              state.dispatch(Action::ChangeSentence { num, sentence });
            })
          };
          let handle_change_derivation = {
            let state = state.clone();
            Callback::from(move |derivation: String| {
              state.dispatch(Action::ChangeDerivation { num, derivation });
            })
          };
          let handle_append_row = {
            let state = state.clone();
            Callback::from(move |_| state.dispatch(Action::Add { after_num: num }))
          };
          html_nested! {
            <component::row::Row
              class={classes!(
                is_last.then_some("border-t-4 border-double border-t-gray-300"),
                consequent_deps.contains(&num).then_some("bg-gray-100"),
              )}
              readonly={props.readonly}
              num={num}
              dependents={dep.nums.clone()}
              is_dependents_complete={dep.is_complete}
              sentence={AttrValue::from(row.sentence.clone())}
              derivation={AttrValue::from(row.derivation.clone())}
              is_derivation_valid={*is_rule_valid}
              on_change_sentence={handle_change_sentence}
              on_change_derivation={handle_change_derivation}
              on_format={handle_format.clone()}
              on_append_row={handle_append_row}
            />
        }}) }
      </tbody>
    </table>
  }
}
