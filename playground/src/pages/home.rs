use yew::{function_component, html, use_state, Html};
use yew_router::hooks::use_location;

use crate::component::table::{Row, Table};

#[function_component(Home)]
pub fn home() -> Html {
  let location = use_location();

  let default_rows = use_state(|| {
    let location = match location {
      Some(location) => location,
      None => return None,
    };
    let hash = match urlencoding::decode(location.hash()) {
      Ok(hash) => hash.trim_start_matches("#").to_owned(),
      Err(_) => return None,
    };
    if hash == "" {
      return None;
    }
    match serde_yaml::from_str::<Vec<Row>>(&hash) {
      Ok(rows) => Some(rows),
      Err(_) => None,
    }
  });

  html! {
    <>
      <section>
        <Table default_value={(*default_rows).clone()} />
      </section>
      <section class="bg-slate-100 px-8 py-1 rounded-2xl mt-8">
        <h4>{"단축키"}</h4>
        <ul>
          <li>{"Shift + Enter : 아래에 행 삽입"}</li>
          <li>{"Enter: 전체 포맷팅"}</li>
        </ul>
        <h4>{"검증 오류"}</h4>
        <ul class="[&_.s]:w-24 [&_.s]:mr-2 [&_.s]:px-[10px] [&_.s]:py-[6px] [&_.s]:bg-white [&_input]:border-b-2 [&_input:focus]:outline-none">
          <li>
            <input type="text" class="s border-b-gray-300 underline decoration-wavy decoration-red-400" value={"(Pa -> a)"} readonly=true />
            {"공통: 빨간 물결표 밑줄— 문법 오류"}
          </li>
          <li>
            <input type="text" class="s border-b-red-400" value={"(Pa -> a)"} readonly=true />
            {"식: 상자 빨간 밑줄 — 문법 오류"}
          </li>
          <li>
            <input type="text" class="s border-b-red-400" value={"1, 2 &I"} readonly=true />
            {"도출규칙: 상자 빨간 밑줄 — 문법 오류 또는 도출규칙 조건 위반"}
          </li>
          <li>
            <div class="s inline-block border-b border-b-red-400">{"1,2,3"}</div>
            {"전제번호: 상자 빨간 밑줄 — 불완전한 전제번호 도출 결과"}
          </li>
        </ul>
      </section>
    </>
  }
}
