mod component;
mod parser;
mod state;

use std::collections::HashSet;

use itertools::izip;
use web_sys::HtmlInputElement;
use yew::{
  classes, events::InputEvent, function_component, html, html_nested, use_reducer, virtual_dom::AttrValue, Callback,
  Html, Properties, TargetCast,
};

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

  let focus_deps = {
    let dep = match state.focused_idx {
      Some(idx) => state.deps_list.get(idx),
      None => None,
    };
    match dep {
      Some(dep) => dep.nums.clone(),
      None => HashSet::new(),
    }
  };

  let handle_format = {
    let state = state.clone();
    Callback::from(move |()| state.dispatch(Action::Format))
  };
  let handle_append_row_at_end = {
    let state = state.clone();
    Callback::from(move |_| {
      state.dispatch(Action::Add {
        after_num: state.rows.len(),
      })
    })
  };

  let handle_change_textbox = {
    let state = state.clone();
    Callback::from(move |e: InputEvent| {
      let target: HtmlInputElement = e.target_unchecked_into();
      state.dispatch(Action::ChangeTextbox { value: target.value() })
    })
  };
  let handle_click_export = {
    let state = state.clone();
    Callback::from(move |_| state.dispatch(Action::ExportToTextbox))
  };
  let handle_click_import = {
    let state = state.clone();
    Callback::from(move |_| state.dispatch(Action::ImportFromTextbox))
  };

  let cls_button = classes!(
    "bg-slate-400",
    "hover:bg-slate-300",
    "text-white",
    "font-bold",
    "px-4",
    "border-b-2",
    "border-slate-500",
    "hover:border-slate-400",
  );
  html! {
    <div>
      <table class="table-fixed font-mono not-prose h-fit">
        <thead>
          <tr class="[&>th]:p-[10px] border-b border-b-gray-400">
            <th class="w-20">{"전제번호"}</th>
            <th class="w-8 text-right">{"#"}</th>
            <th class="">{"식"}</th>
            <th class="w-36">{"도출규칙"}</th>
          </tr>
        </thead>
        <tbody>
          { for izip!(state.rows.iter(), state.deps_list.iter(), state.rule_vaildity_list.iter()).enumerate().map(|(idx, (row, dep, is_rule_valid))| {
            let num = idx + 1;
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
            let handle_focus = {
              let state = state.clone();
              Callback::from(move |_| state.dispatch(Action::ChangeFocus { idx: Some(num - 1) }))
            };
            let handle_blur = {
              let state = state.clone();
              Callback::from(move |_| state.dispatch(Action::ChangeFocus { idx: None }))
            };
            html_nested! {
              <component::row::Row
                class={classes!(
                  focus_deps.contains(&num)
                    .then_some("[&>:nth-child(2)]:font-bold [&>:nth-child(2)]:text-black"),
                  state.focused_idx.map(|focused_idx| (num == (focused_idx + 1))
                    .then_some("[&>:nth-child(1)]:font-bold [&>:nth-child(1)]:text-black")),
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
                on_focus={handle_focus}
                on_blur={handle_blur}
              />
          }}) }
        </tbody>
        if !props.readonly {
          <tfoot>
            <tr>
              <td colspan="4">
                <button class="w-full py-2 font-bold hover:bg-gray-100" onclick={handle_append_row_at_end}>
                {"➕ 행 추가하기 (S-Enter)"}
                </button>
              </td>
            </tr>
          </tfoot>
        }
      </table>
      if !props.readonly {
        <section class="flex flex-col bg-slate-100 p-4 rounded-2xl">
          <div class="flex justify-between items-start mb-2">
            <div class="font-bold">{"텍스트로 내보내기 · 불러오기"}</div>
            <div class="flex justify-end">
              <button class={classes!(cls_button.clone(), "rounded-l")} onclick={handle_click_export}>{"내보내기"}</button>
              <button class={classes!(cls_button.clone(), "rounded-r")} onclick={handle_click_import}>{"불러오기"}</button>
            </div>
          </div>
          <textarea class="font-mono text-xs" rows="4" value={state.textbox.clone()} oninput={handle_change_textbox}></textarea>
        </section>
      }
    </div>
  }
}
