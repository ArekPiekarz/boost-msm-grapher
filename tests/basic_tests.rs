#![allow(non_snake_case)]

mod common;
use common::APP_NAME;

use std::io::Write;


#[test]
fn shouldFail_whenNoFilePathIsProvided()
{
    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().assert().failure()
        .stderr("Error: \"Please provide a path to a file to analyze.\"\n");
}

#[test]
fn shouldFail_whenTooManyArgumentsAreProvided()
{
    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().args(&["file1", "file2"]).assert().failure()
        .stderr("Error: \"Too many arguments passed to program, expected only one with a file path, got 2\"\n");
}

#[test]
fn shouldFail_whenFileCannotBeRead()
{
    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg("wrong_file").assert().failure()
        .stderr("Error: \"Failed to read file: \\\"wrong_file\\\", error: No such file or directory (os error 2)\"\n");
}

#[test]
fn shouldFail_whenFileDoesNotHaveTransitionTable()
{
    let file = tempfile::NamedTempFile::new().unwrap();
    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Transition table was not found.\"\n");
}

#[test]
fn shouldFail_whenTransitionTableHasNoRows()
{
    let cppFileContent = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>

struct StartState : public boost::msm::front::state<> {};
struct TargetState : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = StartState;

    struct transition_table : boost::mpl::vector<> {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event{});
    return 0;
}
"#;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(cppFileContent.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Rows were not found in the transition table.\"\n");
}

#[test]
fn shouldFail_whenFirstRowIdentifierDoesNotEndWithRow()
{
    let cppFileContent = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>

struct StartState : public boost::msm::front::state<> {};
struct TargetState : public boost::msm::front::state<> {};
struct Event {};
struct not_a_row_identifier {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = StartState;

    struct transition_table : boost::mpl::vector<
        not_a_row_identifier
    > {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event{});
    return 0;
}
"#;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(cppFileContent.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Rows were not found in the transition table.\"\n");
}

#[test]
fn shouldFail_whenRowDoesNotHaveTemplateStartSymbol()
{
    let cppFileContent = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>

struct StartState : public boost::msm::front::state<> {};
struct TargetState : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = StartState;

    struct transition_table : boost::mpl::vector<
        _row
    > {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event{});
    return 0;
}
"#;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(cppFileContent.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected row template start, got: TemplateEnd.\"\n");
}

#[test]
fn shouldFail_whenRowIsEmpty()
{
    let cppFileContent = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>

struct StartState : public boost::msm::front::state<> {};
struct TargetState : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = StartState;

    struct transition_table : boost::mpl::vector<
        _row<>
    > {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event{});
    return 0;
}
"#;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(cppFileContent.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected start state, got: TemplateEnd.\"\n");
}

#[test]
fn shouldFail_whenRowDoesNotStartWithIdentifier()
{
    let cppFileContent = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>

struct StartState : public boost::msm::front::state<> {};
struct TargetState : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = StartState;

    struct transition_table : boost::mpl::vector<
        _row<,>
    > {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event{});
    return 0;
}
"#;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(cppFileContent.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected start state, got: Comma.\"\n");
}

#[test]
fn shouldFail_whenRowHasOnlyStartState()
{
    let cppFileContent = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>

struct StartState : public boost::msm::front::state<> {};
struct TargetState : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = StartState;

    struct transition_table : boost::mpl::vector<
        _row<StartState>
    > {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event{});
    return 0;
}
"#;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(cppFileContent.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected comma after start state, got: TemplateEnd.\"\n");
}

#[test]
fn shouldFail_whenRowDoesNotHaveEvent()
{
    let cppFileContent = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>

struct StartState : public boost::msm::front::state<> {};
struct TargetState : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = StartState;

    struct transition_table : boost::mpl::vector<
        _row<StartState,>
    > {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event{});
    return 0;
}
"#;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(cppFileContent.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected event, got: TemplateEnd.\"\n");
}

#[test]
fn shouldFail_whenRowDoesNotHaveCommaAfterEvent()
{
    let cppFileContent = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>

struct StartState : public boost::msm::front::state<> {};
struct TargetState : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = StartState;

    struct transition_table : boost::mpl::vector<
        _row<StartState, Event>
    > {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event{});
    return 0;
}
"#;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(cppFileContent.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected comma after event, got: TemplateEnd.\"\n");
}

#[test]
fn shouldFail_whenRowDoesNotHaveTargetState()
{
    let cppFileContent = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>

struct StartState : public boost::msm::front::state<> {};
struct TargetState : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = StartState;

    struct transition_table : boost::mpl::vector<
        _row<StartState, Event,>
    > {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event{});
    return 0;
}
"#;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(cppFileContent.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected target state, got: TemplateEnd.\"\n");
}

#[test]
fn shouldFail_whenRowHasTargetStateAndDoesNotEndWithCommaOrTemplateEndSymbol()
{
    let cppFileContent = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>

struct StartState : public boost::msm::front::state<> {};
struct TargetState : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = StartState;

    struct transition_table : boost::mpl::vector<
        _row<StartState, Event, TargetState
    {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event{});
    return 0;
}
"#;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(cppFileContent.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected comma or template end symbol after target state, got: BlockStart.\"\n");
}

#[test]
fn shouldFail_whenTransitionTableDoesNotEndWithTemplateEnd()
{
    let cppFileContent = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>

struct StartState : public boost::msm::front::state<> {};
struct TargetState : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = StartState;

    struct transition_table : boost::mpl::vector<
        _row<StartState, Event, TargetState>
    {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event{});
    return 0;
}
"#;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(cppFileContent.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected a comma or a template end after row, got: BlockStart.\"\n");
}

#[test]
fn shouldPass_whenTransitionTableHasRowWithStartStateAndEventAndTargetState()
{
    let cppFileContent = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>

struct StartState : public boost::msm::front::state<> {};
struct TargetState : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = StartState;

    struct transition_table : boost::mpl::vector<
        _row<StartState, Event, TargetState>
    > {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event{});
    return 0;
}
"#;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(cppFileContent.as_bytes()).unwrap();

    let expectedOutput =
"@startuml
hide empty description
[*] --> StartState
StartState --> TargetState : on Event
@enduml
";
    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().success()
        .stdout(expectedOutput);
}

#[test]
fn shouldFail_whenActionRowHasTargetStateAndCommaButNoAction()
{
    let cppFileContent = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>
#include <iostream>

struct StartState : public boost::msm::front::state<> {};
struct TargetState : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    void action(const Event&)
    {
        std::cout << "performing action\n";
    }

    using initial_state = StartState;
    using M = MachineDef;

    struct transition_table : boost::mpl::vector<
        a_row<StartState, Event, TargetState,>
    > {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event{});
    return 0;
}
"#;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(cppFileContent.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected an action, got: TemplateEnd.\"\n");
}

#[test]
fn shouldFail_whenRowHasActionAndDoesNotEndWithCommaOrTemplateEnd()
{
    let cppFileContent = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>
#include <iostream>

struct StartState : public boost::msm::front::state<> {};
struct TargetState : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    void action(const Event&)
    {
        std::cout << "performing action\n";
    }

    using initial_state = StartState;
    using M = MachineDef;

    struct transition_table : boost::mpl::vector<
        a_row<StartState, Event, TargetState, &M::action
    {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event{});
    return 0;
}
"#;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(cppFileContent.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected a comma or a template end after action, got: BlockStart.\"\n");
}

#[test]
fn shouldPass_whenTransitionTableHasRowWithStartStateAndEventAndTargetStateAndAction()
{
    let cppFileContent = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>
#include <iostream>

struct StartState : public boost::msm::front::state<> {};
struct TargetState : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    void action(const Event&)
    {
        std::cout << "performing action\n";
    }

    using initial_state = StartState;
    using M = MachineDef;

    struct transition_table : boost::mpl::vector<
        a_row<StartState, Event, TargetState, &M::action>
    > {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event{});
    return 0;
}
"#;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(cppFileContent.as_bytes()).unwrap();

    let expectedOutput =
r"@startuml
hide empty description
[*] --> StartState
StartState --> TargetState : on Event\ndo &M::action
@enduml
";
    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().success()
        .stdout(expectedOutput);
}

#[test]
fn shouldFail_whenGuardRowHasTargetStateAndCommaButNoGuard()
{
    let cppFileContent = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>

struct StartState : public boost::msm::front::state<> {};
struct TargetState : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    bool guard(const Event&)
    {
        return true;
    }

    using initial_state = StartState;
    using M = MachineDef;

    struct transition_table : boost::mpl::vector<
        g_row<StartState, Event, TargetState,>
    > {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event{});
    return 0;
}
"#;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(cppFileContent.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected a guard, got: TemplateEnd.\"\n");
}

#[test]
fn shouldFail_whenRowHasGuardAndDoesNotEndWithTemplateEndSymbol()
{
    let cppFileContent = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>

struct StartState : public boost::msm::front::state<> {};
struct TargetState : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    bool guard(const Event&)
    {
        return true;
    }

    using initial_state = StartState;
    using M = MachineDef;

    struct transition_table : boost::mpl::vector<
        g_row<StartState, Event, TargetState, &M::guard
    {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event{});
    return 0;
}
"#;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(cppFileContent.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected a template end, got: BlockStart.\"\n");
}

#[test]
fn shouldPass_whenTransitionTableHasRowWithStartStateAndEventAndTargetStateAndGuard()
{
    let cppFileContent = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>

struct StartState : public boost::msm::front::state<> {};
struct TargetState : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    bool guard(const Event&)
    {
        return true;
    }

    using initial_state = StartState;
    using M = MachineDef;

    struct transition_table : boost::mpl::vector<
        g_row<StartState, Event, TargetState, &M::guard>
    > {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event{});
    return 0;
}
"#;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(cppFileContent.as_bytes()).unwrap();

    let expectedOutput =
r"@startuml
hide empty description
[*] --> StartState
StartState --> TargetState : on Event\nif &M::guard
@enduml
";
    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().success()
        .stdout(expectedOutput);
}

#[test]
fn shouldFail_whenRowHasActionAndCommaButNoGuard()
{
    let cppFileContent = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>
#include <iostream>

struct StartState : public boost::msm::front::state<> {};
struct TargetState : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    void action(const Event&)
    {
        std::cout << "performing action\n";
    }

    bool guard(const Event&)
    {
        return true;
    }

    using initial_state = StartState;
    using M = MachineDef;

    struct transition_table : boost::mpl::vector<
        row<StartState, Event, TargetState, &M::action,>
    > {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event{});
    return 0;
}
"#;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(cppFileContent.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected a guard, got: TemplateEnd.\"\n");
}

#[test]
fn shouldFail_whenRowHasActionAndGuardButDoesNotEndWithTemplateEndSymbol()
{
    let cppFileContent = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>
#include <iostream>

struct StartState : public boost::msm::front::state<> {};
struct TargetState : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    void action(const Event&)
    {
        std::cout << "performing action\n";
    }

    bool guard(const Event&)
    {
        return true;
    }

    using initial_state = StartState;
    using M = MachineDef;

    struct transition_table : boost::mpl::vector<
        row<StartState, Event, TargetState, &M::action, &M::guard
    {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event{});
    return 0;
}
"#;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(cppFileContent.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected a template end, got: BlockStart.\"\n");
}

#[test]
fn shouldPass_whenTransitionTableHasRowWithStartStateAndEventAndTargetStateAndActionAndGuard()
{
    let cppFileContent = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>
#include <iostream>

struct StartState : public boost::msm::front::state<> {};
struct TargetState : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    void action(const Event&)
    {
        std::cout << "performing action\n";
    }

    bool guard(const Event&)
    {
        return true;
    }

    using initial_state = StartState;
    using M = MachineDef;

    struct transition_table : boost::mpl::vector<
        row<StartState, Event, TargetState, &M::action, &M::guard>
    > {};
};

using Machine = boost::msm::back::state_machine<MachineDef>;

int main()
{
    Machine machine;
    machine.start();
    machine.process_event(Event{});
    return 0;
}
"#;
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(cppFileContent.as_bytes()).unwrap();

    let expectedOutput =
r"@startuml
hide empty description
[*] --> StartState
StartState --> TargetState : on Event\nif &M::guard\ndo &M::action
@enduml
";
    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().success()
        .stdout(expectedOutput);
}
