#![allow(non_snake_case)]

mod common;
use common::APP_NAME;

use std::io::Write;


#[test]
fn shouldFail_whenRowHasGuardWithNegationAndTemplateStart_butFileEnds()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/euml/operator.hpp>

struct State1 {};
struct State2 {};
struct Event {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;
    template <typename Type>
    using Not = boost::msm::front::euml::Not_<Type>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, None, Not<";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"While parsing a guard, tokens ended prematurely.\"\n");
}

#[test]
fn shouldFail_whenRowHasGuardWithNegationAndTemplateStart_butNoTemplateEnd()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/euml/operator.hpp>

struct State1 {};
struct State2 {};
struct Event {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;
    template <typename Type = void>
    using Not = boost::msm::front::euml::Not_<Type>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, None, Not<
    {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected an identifier or a template end, got: BlockStart.\"\n");
}

#[test]
fn shouldPass_whenRowHasGuardWithNegationWithDefaultType()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/euml/operator.hpp>

struct State1 {};
struct State2 {};
struct Event {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;
    template <typename Type = void>
    using Not = boost::msm::front::euml::Not_<Type>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, None, Not<>>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    let expectedOutput =
r"@startuml
hide empty description
[*] --> State1
State1 --> State2 : on Event\nif Not<>
@enduml
";
    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().success()
        .stdout(expectedOutput);
}

#[test]
fn shouldFail_whenRowHasGuardWithNegationAndInnerType_ButNoTemplateEnd()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/euml/operator.hpp>

struct State1 {};
struct State2 {};
struct Event {};

struct Guard
{
    template <class Fsm, class Event, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&);
};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;
    template <typename Type>
    using Not = boost::msm::front::euml::Not_<Type>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, None, Not<Guard
    {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected a comma, template start or template end, got: BlockStart.\"\n");
}

#[test]
fn shouldFail_whenRowHasGuardWithNegationAndInnerType_ButNoTemplateEndForRow()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/euml/operator.hpp>

struct State1 {};
struct State2 {};
struct Event {};

struct Guard
{
    template <class Fsm, class Event, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&);
};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;
    template <typename Type>
    using Not = boost::msm::front::euml::Not_<Type>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, None, Not<Guard>
    {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected a template end, got: BlockStart.\"\n");
}

#[test]
fn shouldPass_whenRowHasGuardWithNegationOfType()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/euml/operator.hpp>

struct State1 {};
struct State2 {};
struct Event {};

struct Guard
{
    template <class Fsm, class Event, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&);
};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;
    template <typename Type>
    using Not = boost::msm::front::euml::Not_<Type>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, None, Not<Guard>>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    let expectedOutput =
r"@startuml
hide empty description
[*] --> State1
State1 --> State2 : on Event\nif Not<Guard>
@enduml
";
    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().success()
        .stdout(expectedOutput);
}

#[test]
fn shouldFail_whenRowHasGuardWithNegationAndInnerTypeAndTemplateStart_ButNoTemplateEnd()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/euml/operator.hpp>

struct State1 {};
struct State2 {};
struct Event {};

template <class Type = int>
struct Guard
{
    template <class Fsm, class Event, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&);
};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;
    template <typename Type>
    using Not = boost::msm::front::euml::Not_<Type>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, None, Not<Guard<
    {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected an identifier or a template end, got: BlockStart.\"\n");
}

#[test]
fn shouldFail_whenRowHasGuardWithNegationAndInnerTypeWithDefaultType_ButNoTemplateEndForGuard()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/euml/operator.hpp>

struct State1 {};
struct State2 {};
struct Event {};

template <class Type = int>
struct Guard
{
    template <class Fsm, class Event, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&);
};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;
    template <typename Type>
    using Not = boost::msm::front::euml::Not_<Type>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, None, Not<Guard<>
    {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected a comma or a template end, got: BlockStart.\"\n");
}

#[test]
fn shouldFail_whenRowHasGuardWithNegationAndInnerTypeWithDefaultType_ButNoTemplateEndForRow()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/euml/operator.hpp>

struct State1 {};
struct State2 {};
struct Event {};

template <class Type = int>
struct Guard
{
    template <class Fsm, class Event, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&);
};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;
    template <typename Type>
    using Not = boost::msm::front::euml::Not_<Type>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, None, Not<Guard<>>
    {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected a template end, got: BlockStart.\"\n");
}

#[test]
fn shouldFail_whenRowHasGuardWithNegationAndInnerTypeWithDefaultType_ButNoTemplateEndForTable()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/euml/operator.hpp>

struct State1 {};
struct State2 {};
struct Event {};

template <class Type = int>
struct Guard
{
    template <class Fsm, class Event, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&);
};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;
    template <typename Type>
    using Not = boost::msm::front::euml::Not_<Type>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, None, Not<Guard<>>>
    {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected a comma or a template end after row, got: BlockStart.\"\n");
}

#[test]
fn shouldPass_whenRowHasGuardWithNegationAndInnerTypeWithDefaultType()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/euml/operator.hpp>

struct State1 {};
struct State2 {};
struct Event {};

template <class Type = int>
struct Guard
{
    template <class Fsm, class Event, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&);
};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;
    template <typename Type>
    using Not = boost::msm::front::euml::Not_<Type>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, None, Not<Guard<>>>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    let expectedOutput =
r"@startuml
hide empty description
[*] --> State1
State1 --> State2 : on Event\nif Not<Guard<>>
@enduml
";
    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().success()
        .stdout(expectedOutput);
}

#[test]
fn shouldFail_whenRowHasGuardWithConjunctionAndOneInnerTypeAndComma_ButNoSecondType()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/euml/operator.hpp>

struct State1 {};
struct State2 {};
struct Event {};

struct Guard1
{
    template <class Fsm, class Event, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&);
};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;
    template <typename Type1, typename Type2>
    using And = boost::msm::front::euml::And_<Type1, Type2>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, None, And<Guard1,>>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected an identifier, got: TemplateEnd.\"\n");
}

#[test]
fn shouldPass_whenRowHasGuardWithConjunctionOfTwoTypes()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/euml/operator.hpp>

struct State1 {};
struct State2 {};
struct Event {};

struct Guard1
{
    template <class Fsm, class Event, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&);
};

struct Guard2
{
    template <class Fsm, class Event, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&);
};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;
    template <typename Type1, typename Type2>
    using And = boost::msm::front::euml::And_<Type1, Type2>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, None, And<Guard1, Guard2>>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    let expectedOutput =
r"@startuml
hide empty description
[*] --> State1
State1 --> State2 : on Event\nif And<Guard1, Guard2>
@enduml
";
    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().success()
        .stdout(expectedOutput);
}
