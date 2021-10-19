#![allow(non_snake_case)]

mod common;
use common::APP_NAME;

use std::io::Write;


#[test]
fn shouldFail_whenRowHasActionSequenceWithTemplateStart_butFileEnds()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/functor_row.hpp>

struct State1 {};
struct State2 {};
struct Event {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    template <typename ...Actions>
    using ActionSequence = boost::msm::front::ActionSequence_<boost::mpl::vector<Actions...>>;
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, ActionSequence<";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"While parsing an action, tokens ended prematurely.\"\n");
}

#[test]
fn shouldFail_whenRowHasActionSequenceWithTemplateStart_butNoTemplateEnd()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/functor_row.hpp>

struct State1 {};
struct State2 {};
struct Event {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    template <typename ...Actions>
    using ActionSequence = boost::msm::front::ActionSequence_<boost::mpl::vector<Actions...>>;
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, ActionSequence<
    {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected an identifier or a template end, got: BlockStart.\"\n");
}

#[test]
fn shouldPass_whenRowHasActionSequenceWithDefaultTemplateType()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/functor_row.hpp>

struct State1 {};
struct State2 {};
struct Event {};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    template <typename ...Actions>
    using ActionSequence = boost::msm::front::ActionSequence_<boost::mpl::vector<Actions...>>;
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, ActionSequence<>>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    let expectedOutput =
r"@startuml
hide empty description
[*] --> State1
State1 --> State2 : on Event\ndo ActionSequence<>
@enduml
";
    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().success()
        .stdout(expectedOutput);
}

#[test]
fn shouldFail_whenRowHasActionSequenceWithOneAction_butNoTemplateEnd()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/functor_row.hpp>

struct State1 {};
struct State2 {};
struct Event {};

struct Action
{
    template <class Event, class Fsm, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&);
};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    template <typename ...Actions>
    using ActionSequence = boost::msm::front::ActionSequence_<boost::mpl::vector<Actions...>>;
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, ActionSequence<Action
    {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected a comma, template start or template end, got: BlockStart.\"\n");
}

#[test]
fn shouldFail_whenRowHasActionSequenceWithOneAction_butNoTemplateEndForRow()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/functor_row.hpp>

struct State1 {};
struct State2 {};
struct Event {};

struct Action
{
    template <class Event, class Fsm, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&);
};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    template <typename ...Actions>
    using ActionSequence = boost::msm::front::ActionSequence_<boost::mpl::vector<Actions...>>;
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, ActionSequence<Action>
    {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected a comma or a template end after action, got: BlockStart.\"\n");
}

#[test]
fn shouldPass_whenRowHasActionSequenceWithOneAction()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/functor_row.hpp>

struct State1 {};
struct State2 {};
struct Event {};

struct Action
{
    template <class Event, class Fsm, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&);
};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    template <typename ...Actions>
    using ActionSequence = boost::msm::front::ActionSequence_<boost::mpl::vector<Actions...>>;
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, ActionSequence<Action>>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    let expectedOutput =
r"@startuml
hide empty description
[*] --> State1
State1 --> State2 : on Event\ndo ActionSequence<Action>
@enduml
";
    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().success()
        .stdout(expectedOutput);
}

#[test]
fn shouldFail_whenRowHasActionSequenceWithOneActionWithTemplateStart_butNoTemplateEnd()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/functor_row.hpp>

struct State1 {};
struct State2 {};
struct Event {};

template <class Type = int>
struct Action
{
    template <class Event, class Fsm, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&);
};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    template <typename ...Actions>
    using ActionSequence = boost::msm::front::ActionSequence_<boost::mpl::vector<Actions...>>;
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, ActionSequence<Action<
    {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected an identifier or a template end, got: BlockStart.\"\n");
}

#[test]
fn shouldPass_whenRowHasActionSequenceWithOneActionWithDefaultTemplateType()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/functor_row.hpp>

struct State1 {};
struct State2 {};
struct Event {};

template <class Type = int>
struct Action
{
    template <class Event, class Fsm, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&);
};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    template <typename ...Actions>
    using ActionSequence = boost::msm::front::ActionSequence_<boost::mpl::vector<Actions...>>;
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, ActionSequence<Action<>>>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    let expectedOutput =
r"@startuml
hide empty description
[*] --> State1
State1 --> State2 : on Event\ndo ActionSequence<Action<>>
@enduml
";
    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().success()
        .stdout(expectedOutput);
}

#[test]
fn shouldFail_whenRowHasActionSequenceWithOneActionAndComma_butNoSecondAction()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/functor_row.hpp>

struct State1 {};
struct State2 {};
struct Event {};

struct Action
{
    template <class Event, class Fsm, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&);
};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    template <typename ...Actions>
    using ActionSequence = boost::msm::front::ActionSequence_<boost::mpl::vector<Actions...>>;
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, ActionSequence<Action,>>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected an identifier, got: TemplateEnd.\"\n");
}

#[test]
fn shouldPass_whenRowHasActionSequenceWithTwoActions()
{
    let transitionTable =
"#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/functor_row.hpp>

struct State1 {};
struct State2 {};
struct Event {};

struct Action1
{
    template <class Event, class Fsm, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&);
};

struct Action2
{
    template <class Event, class Fsm, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&);
};

struct Machine : public boost::msm::front::state_machine_def<Machine>
{
    template <typename ...Actions>
    using ActionSequence = boost::msm::front::ActionSequence_<boost::mpl::vector<Actions...>>;
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, ActionSequence<Action1, Action2>>
    > {};
};";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    let expectedOutput =
r"@startuml
hide empty description
[*] --> State1
State1 --> State2 : on Event\ndo ActionSequence<Action1, Action2>
@enduml
";
    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().success()
        .stdout(expectedOutput);
}
