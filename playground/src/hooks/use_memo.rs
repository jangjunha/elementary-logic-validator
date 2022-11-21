// https://github.com/yewstack/yew/blob/5355b65ff5f9747cbad801d4b337a5ac7a94d0f4/packages/yew/src/functional/hooks/use_memo.rs

use yew::use_mut_ref;

fn use_memo_base<T, F, D, K>(f: F, deps: D) -> std::rc::Rc<T>
where
  T: 'static,
  F: FnOnce(D) -> (T, K),
  K: 'static + std::borrow::Borrow<D>,
  D: PartialEq,
{
  struct MemoState<T, K> {
    memo_key: K,
    result: std::rc::Rc<T>,
  }
  let state = use_mut_ref(|| -> Option<MemoState<T, K>> { None });

  let mut state = state.borrow_mut();
  match &*state {
    Some(existing) if existing.memo_key.borrow() != &deps => {
      *state = None;
    }
    _ => {}
  };
  let state = state.get_or_insert_with(|| {
    let (result, memo_key) = f(deps);
    let result = std::rc::Rc::new(result);
    MemoState { memo_key, result }
  });
  state.result.clone()
}

pub fn use_memo<T, F, D>(f: F, deps: D) -> std::rc::Rc<T>
where
  T: 'static,
  F: FnOnce(&D) -> T,
  D: 'static + PartialEq,
{
  use_memo_base(|d| (f(&d), d), deps)
}
