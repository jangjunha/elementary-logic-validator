use yew::{function_component, html, Html};

use crate::component::table::Table;

#[function_component(Home)]
pub fn home() -> Html {
  html! {
    <>
      <section>
        <Table />
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
