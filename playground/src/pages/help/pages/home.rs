use lazy_static::lazy_static;
use yew::{function_component, html, html_nested, Html};

use crate::component::table::{Row, Table};

struct Sample {
  pub title: &'static str,
  pub rows: Vec<Row>,
}

lazy_static! {
  static ref SAMPLES: Vec<Sample> = vec![
    Sample {
      title: "연언 도입 규칙",
      rows: vec![
        Row {
          sentence: "P".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "Q".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "(P & Q)".to_owned(),
          derivation: "1, 2 &I".to_owned(),
        },
      ],
    },
    Sample {
      title: "연언 제거 규칙",
      rows: vec![
        Row {
          sentence: "(P & Q)".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "P".to_owned(),
          derivation: "1 &E".to_owned(),
        }
      ],
    },
    Sample {
      title: "선언 도입 규칙",
      rows: vec![
        Row {
          sentence: "P".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "(P ∨ Q)".to_owned(),
          derivation: "1 |I".to_owned(),
        },
      ],
    },
    Sample {
      title: "선언 제거 규칙(∨E), 조건문 도입 규칙(->I)",
      rows: vec![
        Row {
          sentence: "(P ∨ Q)".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "-P".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "P".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "\\bot".to_owned(),
          derivation: "2, 3 ->E".to_owned(),
        },
        Row {
          sentence: "Q".to_owned(),
          derivation: "4 \\bot".to_owned(),
        },
        Row {
          sentence: "Q".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "Q".to_owned(),
          derivation: "1, 3-5, 6-6 |E".to_owned(),
        },
        Row {
          sentence: "(-P -> Q)".to_owned(),
          derivation: "2-7 ->I".to_owned(),
        },
      ],
    },
    Sample {
      title: "조건문 제거 규칙",
      rows: vec![
        Row {
          sentence: "(P -> Q)".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "P".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "Q".to_owned(),
          derivation: "1, 2 ->E".to_owned(),
        },
      ],
    },
    Sample {
      title: "⊥ 규칙",
      rows: vec![
        Row {
          sentence: "⊥".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "A".to_owned(),
          derivation: "1 \\bot".to_owned(),
        },
        Row {
          sentence: "B".to_owned(),
          derivation: "1 \\bot".to_owned(),
        },
      ],
    },
    Sample {
      title: "부정 도입 규칙",
      rows: vec![
        Row {
          sentence: "-(P -> Q)".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "Q".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "(P -> Q)".to_owned(),
          derivation: "2 ->I".to_owned(),
        },
        Row {
          sentence: "\\bot".to_owned(),
          derivation: "1,3 ->E".to_owned(),
        },
        Row {
          sentence: "-Q".to_owned(),
          derivation: "2-4 -I".to_owned(),
        },
      ],
    },
    Sample {
      title: "부정 제거 규칙",
      rows: vec![
        Row {
          sentence: "-(A -> -B)".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "-A".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "A".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "\\bot".to_owned(),
          derivation: "2, 3 ->E".to_owned(),
        },
        Row {
          sentence: "-B".to_owned(),
          derivation: "4 \\bot".to_owned(),
        },
        Row {
          sentence: "A -> -B".to_owned(),
          derivation: "3-5 ->I".to_owned(),
        },
        Row {
          sentence: "\\bot".to_owned(),
          derivation: "1, 6 ->E".to_owned(),
        },
        Row {
          sentence: "A".to_owned(),
          derivation: "2-7 -E".to_owned(),
        },
      ],
    },
    Sample {
      title: "쌍조건문 도입 규칙",
      rows: vec![
        Row {
          sentence: "(P -> Q)".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "(Q -> P)".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "(P <-> Q)".to_owned(),
          derivation: "1, 2 <->I".to_owned(),
        },
      ],
    },
    Sample {
      title: "쌍조건문 제거 규칙",
      rows: vec![
        Row {
          sentence: "(P <-> Q)".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "P -> Q".to_owned(),
          derivation: "1 <->E".to_owned(),
        },
        Row {
          sentence: "Q -> P".to_owned(),
          derivation: "1 <->E".to_owned(),
        },
      ],
    },
    Sample {
      title: "보편양화사 도입 규칙 1",
      rows: vec![
        Row {
          sentence: "Fa & Ga".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "Fa".to_owned(),
          derivation: "1 &E".to_owned(),
        },
        Row {
          sentence: "(Fa & Ga) -> Fa".to_owned(),
          derivation: "1-2 ->I".to_owned(),
        },
        Row {
          sentence: "(x)((Fx & Gx) -> Fx)".to_owned(),
          derivation: "3 ()I".to_owned(),
        },
      ],
    },
    Sample {
      title: "보편양화사 도입 규칙 2",
      rows: vec![
        Row {
          sentence: "(]y)(x)Lxy".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "(x)Lxb".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "Lab".to_owned(),
          derivation: "2 ()E".to_owned(),
        },
        Row {
          sentence: "(]y)Lay".to_owned(),
          derivation: "3 ]I".to_owned(),
        },
        Row {
          sentence: "(]y)Lay".to_owned(),
          derivation: "1, 2-4 ]E".to_owned(),
        },
        Row {
          sentence: "(x)(]y)Lxy".to_owned(),
          derivation: "5 ()I".to_owned(),
        },
      ],
    },
    Sample {
      title: "보편양화사 제거 규칙",
      rows: vec![
        Row {
          sentence: "(x)Px".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "Pa".to_owned(),
          derivation: "1 ()E".to_owned(),
        },
      ],
    },
    Sample {
      title: "존재양화사 도입 규칙",
      rows: vec![
        Row {
          sentence: "Pa".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "(]x)Px".to_owned(),
          derivation: "1 ]I".to_owned(),
        },
      ],
    },
    Sample {
      title: "존재양화사 제거 규칙",
      rows: vec![
        Row {
          sentence: "(]x)Px".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "(x)(Px -> Qx)".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "(Pa -> Qa)".to_owned(),
          derivation: "2 ()E".to_owned(),
        },
        Row {
          sentence: "Pa".to_owned(),
          derivation: "P".to_owned(),
        },
        Row {
          sentence: "Qa".to_owned(),
          derivation: "3, 4 ->E".to_owned(),
        },
        Row {
          sentence: "(]x)Qx".to_owned(),
          derivation: "5 ]I".to_owned(),
        },
        Row {
          sentence: "(]x)Qx".to_owned(),
          derivation: "1, 4-6 ]E".to_owned(),
        },
      ],
    },
  ];
}

#[function_component(Home)]
pub fn home() -> Html {
  html! {
    <>
      <section>
        <p>
          {"형식언어 ℒ과 Gentzen의 추론 규칙에 따른 논증이 타당한지 실시간으로 검증하는 기능을 가진 웹페이지입니다."}
        </p>
        <p>
          {"고려대학교 〈기호논리학〉 수업과 교재 "}
          <a href="https://product.kyobobook.co.kr/detail/S000000548655" target="_blank" rel="noopener noreferrer">{"〈기호논리학〉"}</a>
          {"(Benson Mates, 김영정·선우환 역, 문예출판사, 1995)를 바탕으로 합니다."}
          {"〈기호논리학〉, 〈프로그래밍언어〉 수업을 듣고 실습해보면서 만든 사이트입니다."}
        </p>
        <p>
          {"파서, 검증기 및 웹사이트 소스코드는 "}<a href="https://github.com/jangjunha/elementary-logic-validator" target="_blank" rel="noopener noreferrer">{"https://github.com/jangjunha/elementary-logic-validator"}</a>{"에 공개되어 있습니다."}
        </p>
        <p>
          {"만든이는 "}<a href="https://jangjunha.me/">{"jangjunha"}</a>{"입니다."}
        </p>
      </section>

      <section>
        <h2>{"언어 L"}</h2>
        <p>{"TODO:"}</p>
      </section>

      <section>
        <h2>{"기호 입력 대체"}</h2>
        <p>{"키보드로 입력할 수 없는 문자들을 대체 기호로 바꿔 입력할 수 있습니다. 입력 후 엔터 키를 누르면 포맷팅이 실행되며 자동으로 대표 기호로 변환됩니다."}</p>
        <table>
          <thead>
            <tr>
              <th>{"이름"}</th>
              <th>{"대표 기호"}</th>
              <th>{"대체 기호"}</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>{"연언"}</td>
              <td><code>{"&"}</code></td>
              <td></td>
            </tr>
            <tr>
              <td>{"선언"}</td>
              <td><code>{"∨"}</code>{"(U+2228 Logical Or)"}</td>
              <td>
                <code>{"|"}</code>
              </td>
            </tr>
            <tr>
              <td>{"부정"}</td>
              <td><code>{"¬"}</code></td>
              <td>
                <code>{"-"}</code>
              </td>
            </tr>
            <tr>
              <td>{"화살표"}</td>
              <td><code>{"→"}</code></td>
              <td>
                <code>{"->"}</code>
              </td>
            </tr>
            <tr>
              <td>{"쌍화살표"}</td>
              <td><code>{"↔"}</code></td>
              <td>
                <code>{"<->"}</code>
              </td>
            </tr>
            <tr>
              <td>{"존재양화사"}</td>
              <td><code>{"∃"}</code></td>
              <td>
                <code>{"]"}</code>
              </td>
            </tr>
            <tr>
              <td>{"Falsum"}</td>
              <td><code>{"⊥"}</code></td>
              <td>
                <code>{"\\bot"}</code>
              </td>
            </tr>
          </tbody>
        </table>
      </section>

      <section>
        <h2>{"추론 규칙들과 추론 규칙 명세 언어:"}</h2>
        { for SAMPLES.iter().map(|Sample { title, rows }| {
            html_nested! {
              <section>
                <h3>{title}</h3>
                <Table default_value={rows.clone()} readonly=true />
              </section>
            }
        })}
      </section>
    </>
  }
}
