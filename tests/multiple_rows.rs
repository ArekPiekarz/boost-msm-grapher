#![allow(non_snake_case)]

use std::io::Write;

#[test]
fn shouldFail_whenThereIsRowAndCommaAfterItButNoSecondRow()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct state1 {};
struct state2 {};
struct event {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    struct transition_table : boost::mpl::vector<
        _row<state1, event, state2>,
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected row identifier, got: TemplateEnd.\"\n");
}

#[test]
fn shouldPass_whenTableHasTwoSimpleRows()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct state1 {};
struct state2 {};
struct event1 {};
struct event2 {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    struct transition_table : boost::mpl::vector<
        _row<state1, event1, state2>,
        _row<state2, event2, state1>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    let expectedOutput =
r"@startuml
hide empty description
[*] --> state1
state1 --> state2 : on event1
state2 --> state1 : on event2
@enduml
";
    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().success()
        .stdout(expectedOutput);
}

#[test]
fn shouldPass_whenTableHasFourDifferentKindsOfRows()
{
        let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct state1 {};
struct state2 {};
struct state3 {};
struct state4 {};
struct event1 {};
struct event2 {};
struct event3 {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    void action1(const event2&);
    void action2(const event3&);
    bool guard1(const event2&);
    bool guard2(const event3&);

    struct transition_table : boost::mpl::vector<
         _row<state1, event1, state2>,
        a_row<state2, event2, state3, &Machine::action1>,
        g_row<state2, event2, state4, &Machine::guard1>,
          row<state3, event3, state1, &Machine::action2, &Machine::guard2>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    let expectedOutput =
r"@startuml
hide empty description
[*] --> state1
state1 --> state2 : on event1
state2 --> state3 : on event2\ndo &Machine::action1
state2 --> state4 : on event2\nif &Machine::guard1
state3 --> state1 : on event3\nif &Machine::guard2\ndo &Machine::action2
@enduml
";
    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().success()
        .stdout(expectedOutput);
}
