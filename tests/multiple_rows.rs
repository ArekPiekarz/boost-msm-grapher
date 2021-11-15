#![allow(non_snake_case)]

mod common;
use common::APP_NAME;

use std::io::Write;

#[test]
fn shouldFail_whenThereIsRowAndCommaAfterItButNoSecondRow()
{
    let transitionTable = "
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>

struct State1 : public boost::msm::front::state<> {};
struct State2 : public boost::msm::front::state<> {};
struct Event1 {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = State1;

    struct transition_table : boost::mpl::vector<
        _row<State1, Event1, State2>,
    > {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event1{});
    return 0;
}
";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected row identifier, got: TemplateEnd.\"\n");
}

#[test]
fn shouldPass_whenTableHasTwoSimpleRows()
{
    let transitionTable = "
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>

struct State1 : public boost::msm::front::state<> {};
struct State2 : public boost::msm::front::state<> {};
struct Event1 {};
struct Event2 {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = State1;

    struct transition_table : boost::mpl::vector<
        _row<State1, Event1, State2>,
        _row<State2, Event2, State1>
    > {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event1{});
    machine.process_event(Event2{});
    return 0;
}
";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    let expectedOutput =
r"@startuml
hide empty description
[*] --> State1
State1 --> State2 : on Event1
State2 --> State1 : on Event2
@enduml
";
    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().success()
        .stdout(expectedOutput);
}

#[test]
fn shouldPass_whenTableHasFourDifferentKindsOfRows()
{
        let transitionTable = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>
#include <iostream>

struct State1 : public boost::msm::front::state<> {};
struct State2 : public boost::msm::front::state<> {};
struct State3 : public boost::msm::front::state<> {};
struct State4 : public boost::msm::front::state<> {};
struct Event1 {};
struct Event2 {};
struct Event3 {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    void action1(const Event2&)
    {
        std::cout << "performing action1\n";
    }

    void action2(const Event3&)
    {
        std::cout << "performing action2\n";
    }

    bool guard1(const Event2&)
    {
        return false;
    }

    bool guard2(const Event3&)
    {
        return true;
    }

    using M = MachineDef;
    using initial_state = State1;

    struct transition_table : boost::mpl::vector<
         _row<State1, Event1, State2>,
        a_row<State2, Event2, State3, &M::action1>,
        g_row<State2, Event2, State4, &M::guard1>,
          row<State3, Event3, State1, &M::action2, &M::guard2>
    > {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event1{});
    machine.process_event(Event2{});
    machine.process_event(Event3{});
    return 0;
}
"#;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    let expectedOutput =
r"@startuml
hide empty description
[*] --> State1
State1 --> State2 : on Event1
State2 --> State3 : on Event2\ndo &M::action1
State2 --> State4 : on Event2\nif &M::guard1
State3 --> State1 : on Event3\nif &M::guard2\ndo &M::action2
@enduml
";
    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().success()
        .stdout(expectedOutput);
}
