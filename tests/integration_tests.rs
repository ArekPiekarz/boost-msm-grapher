#![allow(non_snake_case)]

use std::io::Write;


#[test]
fn shouldFail_whenNoFilePathIsProvided()
{
    assert_cmd::Command::cargo_bin("msm_graph").unwrap().assert().failure()
        .stderr("Error: \"Please provide a path to a file to analyze.\"\n");
}

#[test]
fn shouldFail_whenTooManyArgumentsAreProvided()
{
    assert_cmd::Command::cargo_bin("msm_graph").unwrap().args(&["file1", "file2"]).assert().failure()
        .stderr("Error: \"Too many arguments passed to program, expected only one with a file path, got 2\"\n");
}

#[test]
fn shouldFail_whenFileCannotBeRead()
{
    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg("wrong_file").assert().failure()
        .stderr("Error: \"Failed to read file: \\\"wrong_file\\\", error: No such file or directory (os error 2)\"\n");
}

#[test]
fn shouldFail_whenFileDoesNotHaveTransitionTable()
{
    let file = tempfile::NamedTempFile::new().unwrap();
    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Transition table was not found.\"\n");
}

#[test]
fn shouldFail_whenTransitionTableHasNoRows()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    struct transition_table : boost::mpl::vector<> {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Rows were not found in the transition table.\"\n");
}

#[test]
fn shouldFail_whenFirstRowIdentifierDoesNotEndWithRow()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct not_a_row_identifier {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    struct transition_table : boost::mpl::vector<
        not_a_row_identifier
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Rows were not found in the transition table.\"\n");
}

#[test]
fn shouldFail_whenRowDoesNotHaveTemplateStartSymbol()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct Row {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    struct transition_table : boost::mpl::vector<
        Row
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected row template start, got: TemplateEnd.\"\n");
}

#[test]
fn shouldFail_whenRowIsEmpty()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    struct transition_table : boost::mpl::vector<
        _row<>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected start state, got: TemplateEnd.\"\n");
}

#[test]
fn shouldFail_whenRowDoesNotStartWithIdentifier()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    struct transition_table : boost::mpl::vector<
        _row<,>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected start state, got: Comma.\"\n");
}

#[test]
fn shouldFail_whenRowHasOnlyStartState()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct start_state {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    struct transition_table : boost::mpl::vector<
        _row<start_state>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected comma after start state, got: TemplateEnd.\"\n");
}

#[test]
fn shouldFail_whenRowDoesNotHaveEvent()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct start_state {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    struct transition_table : boost::mpl::vector<
        _row<start_state,>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected event, got: TemplateEnd.\"\n");
}

#[test]
fn shouldFail_whenRowDoesNotHaveCommaAfterEvent()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct start_state {};
struct event {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    struct transition_table : boost::mpl::vector<
        _row<start_state, event>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected comma after event, got: TemplateEnd.\"\n");
}

#[test]
fn shouldFail_whenRowDoesNotHaveTargetState()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct start_state {};
struct event {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    struct transition_table : boost::mpl::vector<
        _row<start_state, event,>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected target state, got: TemplateEnd.\"\n");
}

#[test]
fn shouldFail_whenRowHasTargetStateAndDoesNotEndWithCommaOrTemplateEndSymbol()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct start_state {};
struct event {};
struct target_state {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    struct transition_table : boost::mpl::vector<
        _row<start_state, event, target_state
    {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected comma or template end symbol after target state, got: BlockStart.\"\n");
}

#[test]
fn shouldFail_whenTransitionTableDoesNotEndWithTemplateEndSymbol()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct start_state {};
struct event {};
struct target_state {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    struct transition_table : boost::mpl::vector<
        _row<start_state, event, target_state>
    {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected comma or template end symbol after row, got: BlockStart.\"\n");
}

#[test]
fn shouldPass_whenTransitionTableHasRowWithStartStateAndEventAndTargetState()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct start_state {};
struct event {};
struct target_state {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    struct transition_table : boost::mpl::vector<
        _row<start_state, event, target_state>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    let expectedOutput =
"@startuml
hide empty description
[*] --> start_state
start_state --> target_state : on event
@enduml
";
    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().success()
        .stdout(expectedOutput);
}

#[test]
fn shouldFail_whenActionRowHasTargetStateAndCommaButNoAction()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct start_state {};
struct event {};
struct target_state {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    struct transition_table : boost::mpl::vector<
        a_row<start_state, event, target_state,>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected action, got: TemplateEnd.\"\n");
}

#[test]
fn shouldFail_whenRowHasActionAndDoesNotEndWithCommaOrTemplateEndSymbol()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct start_state {};
struct event {};
struct target_state {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    void action(const event&);

    struct transition_table : boost::mpl::vector<
        a_row<start_state, event, target_state, &Machine::action
    {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected comma or template end symbol after action, got: BlockStart.\"\n");
}

#[test]
fn shouldPass_whenTransitionTableHasRowWithStartStateAndEventAndTargetStateAndAction()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct start_state {};
struct event {};
struct target_state {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    void action(const event&);

    struct transition_table : boost::mpl::vector<
        a_row<start_state, event, target_state, &Machine::action>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    let expectedOutput =
r"@startuml
hide empty description
[*] --> start_state
start_state --> target_state : on event\ndo &Machine::action
@enduml
";
    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().success()
        .stdout(expectedOutput);
}

#[test]
fn shouldFail_whenGuardRowHasTargetStateAndCommaButNoGuard()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct start_state {};
struct event {};
struct target_state {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    struct transition_table : boost::mpl::vector<
        g_row<start_state, event, target_state,>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected guard, got: TemplateEnd.\"\n");
}

#[test]
fn shouldFail_whenRowHasGuardAndDoesNotEndWithTemplateEndSymbol()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct start_state {};
struct event {};
struct target_state {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    bool guard(const event&);

    struct transition_table : boost::mpl::vector<
        g_row<start_state, event, target_state, &Machine::guard
    {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected template end symbol, got: BlockStart.\"\n");
}

#[test]
fn shouldPass_whenTransitionTableHasRowWithStartStateAndEventAndTargetStateAndGuard()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct start_state {};
struct event {};
struct target_state {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    bool guard(const event&);

    struct transition_table : boost::mpl::vector<
        g_row<start_state, event, target_state, &Machine::guard>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    let expectedOutput =
r"@startuml
hide empty description
[*] --> start_state
start_state --> target_state : on event\nif &Machine::guard
@enduml
";
    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().success()
        .stdout(expectedOutput);
}

#[test]
fn shouldFail_whenRowHasActionAndCommaButNoGuard()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct start_state {};
struct event {};
struct target_state {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    void action(const event&);

    struct transition_table : boost::mpl::vector<
        row<start_state, event, target_state, &Machine::action,>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected guard, got: TemplateEnd.\"\n");
}

#[test]
fn shouldFail_whenRowHasActionAndGuardButDoesNotEndWithTemplateEndSymbol()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct start_state {};
struct event {};
struct target_state {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    void action(const event&);
    bool guard(const event&);

    struct transition_table : boost::mpl::vector<
        row<start_state, event, target_state, &Machine::action, &Machine::guard
    {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected template end symbol, got: BlockStart.\"\n");
}

#[test]
fn shouldPass_whenTransitionTableHasRowWithStartStateAndEventAndTargetStateAndActionAndGuard()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>

struct start_state {};
struct event {};
struct target_state {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    void action(const event&);
    bool guard(const event&);

    struct transition_table : boost::mpl::vector<
        row<start_state, event, target_state, &Machine::action, &Machine::guard>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    let expectedOutput =
r"@startuml
hide empty description
[*] --> start_state
start_state --> target_state : on event\nif &Machine::guard\ndo &Machine::action
@enduml
";
    assert_cmd::Command::cargo_bin("msm_graph").unwrap().arg(file.path()).assert().success()
        .stdout(expectedOutput);
}
