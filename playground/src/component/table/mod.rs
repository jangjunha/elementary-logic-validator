mod component;
mod state;

use yew::{function_component, html, use_reducer, virtual_dom::AttrValue, Callback};

use self::state::{Action, State};

#[function_component(Table)]
pub fn table() -> Html {
  let state = use_reducer(|| State::init());

  let handle_format = {
    let state = state.clone();
    Callback::from(move |()| state.dispatch(Action::Format))
  };

  html! {
    <table class="table-fixed w-full">
      <thead>
        <tr>
          <th class="w-24">{"전제번호"}</th>
          <th class="w-8">{"#"}</th>
          <th class="">{"식"}</th>
          <th class="w-32">{"도출규칙"}</th>
        </tr>
      </thead>
      <tbody>
        { for std::iter::zip(state.rows.iter(), state.deps_list.iter()).enumerate().map(|(idx, (row, dep))| {
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
          html! {
            <component::row::Row
              num={num}
              dependents={dep.nums.clone()}
              is_dependents_complete={dep.is_complete}
              sentence={AttrValue::from(row.sentence.clone())}
              derivation={AttrValue::from(row.derivation.clone())}
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
