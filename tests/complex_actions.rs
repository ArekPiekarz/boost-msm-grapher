#![allow(non_snake_case)]

mod common;
use common::APP_NAME;

use std::io::Write;


#[test]
fn shouldFail_whenRowHasActionSequenceWithTemplateStart_butFileEnds()
{
    let transitionTable = "
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/functor_row.hpp>

struct State1 : public boost::msm::front::state<> {};
struct State2 : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = State1;
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
    let transitionTable = "
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/functor_row.hpp>

struct State1 : public boost::msm::front::state<> {};
struct State2 : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = State1;
    template <typename ...Actions>
    using ActionSequence = boost::msm::front::ActionSequence_<boost::mpl::vector<Actions...>>;
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, ActionSequence<
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
";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected an identifier or a template end, got: BlockStart.\"\n");
}

#[test]
fn shouldPass_whenRowHasActionSequenceWithNoActions()
{
    let transitionTable = "
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/functor_row.hpp>

struct State1 : public boost::msm::front::state<> {};
struct State2 : public boost::msm::front::state<> {};
struct Event {};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = State1;
    template <typename ...Actions>
    using ActionSequence = boost::msm::front::ActionSequence_<boost::mpl::vector<Actions...>>;
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, ActionSequence<>>
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
";
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
    let transitionTable = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/functor_row.hpp>
#include <iostream>

struct State1 : public boost::msm::front::state<> {};
struct State2 : public boost::msm::front::state<> {};
struct Event {};

struct Action
{
    template <class Event, class Fsm, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&)
    {
        std::cout << "performing Action\n";
    }
};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = State1;
    template <typename ...Actions>
    using ActionSequence = boost::msm::front::ActionSequence_<boost::mpl::vector<Actions...>>;
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, ActionSequence<Action
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
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected a comma, template start or template end, got: BlockStart.\"\n");
}

#[test]
fn shouldFail_whenRowHasActionSequenceWithOneAction_butNoTemplateEndForRow()
{
    let transitionTable = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/functor_row.hpp>
#include <iostream>

struct State1 : public boost::msm::front::state<> {};
struct State2 : public boost::msm::front::state<> {};
struct Event {};

struct Action
{
    template <class Event, class Fsm, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&)
    {
        std::cout << "performing Action\n";
    }
};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = State1;
    template <typename ...Actions>
    using ActionSequence = boost::msm::front::ActionSequence_<boost::mpl::vector<Actions...>>;
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, ActionSequence<Action>
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
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected a comma or a template end after action, got: BlockStart.\"\n");
}

#[test]
fn shouldPass_whenRowHasActionSequenceWithOneAction()
{
    let transitionTable = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/functor_row.hpp>
#include <iostream>

struct State1 : public boost::msm::front::state<> {};
struct State2 : public boost::msm::front::state<> {};
struct Event {};

struct Action
{
    template <class Event, class Fsm, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&)
    {
        std::cout << "performing Action\n";
    }
};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = State1;
    template <typename ...Actions>
    using ActionSequence = boost::msm::front::ActionSequence_<boost::mpl::vector<Actions...>>;
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, ActionSequence<Action>>
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
    let transitionTable = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/functor_row.hpp>
#include <iostream>

struct State1 : public boost::msm::front::state<> {};
struct State2 : public boost::msm::front::state<> {};
struct Event {};

template <class Type = int>
struct Action
{
    template <class Event, class Fsm, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&)
    {
        std::cout << "performing Action\n";
    }
};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = State1;
    template <typename ...Actions>
    using ActionSequence = boost::msm::front::ActionSequence_<boost::mpl::vector<Actions...>>;
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, ActionSequence<Action<
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
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected an identifier or a template end, got: BlockStart.\"\n");
}

#[test]
fn shouldPass_whenRowHasActionSequenceWithOneActionWithDefaultTemplateType()
{
    let transitionTable = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/functor_row.hpp>
#include <iostream>

struct State1 : public boost::msm::front::state<> {};
struct State2 : public boost::msm::front::state<> {};
struct Event {};

template <class Type = int>
struct Action
{
    template <class Event, class Fsm, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&)
    {
        std::cout << "performing Action\n";
    }
};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = State1;
    template <typename ...Actions>
    using ActionSequence = boost::msm::front::ActionSequence_<boost::mpl::vector<Actions...>>;
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, ActionSequence<Action<>>>
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
    let transitionTable = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/functor_row.hpp>
#include <iostream>

struct State1 : public boost::msm::front::state<> {};
struct State2 : public boost::msm::front::state<> {};
struct Event {};

struct Action1
{
    template <class Event, class Fsm, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&)
    {
        std::cout << "performing Action1\n";
    }
};

struct Action2
{
    template <class Event, class Fsm, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&)
    {
        std::cout << "performing Action2\n";
    }
};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = State1;
    template <typename ...Actions>
    using ActionSequence = boost::msm::front::ActionSequence_<boost::mpl::vector<Actions...>>;
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, ActionSequence<Action1,>>
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
    file.write_all(transitionTable.as_bytes()).unwrap();

    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(file.path()).assert().failure()
        .stderr("Error: \"Expected an identifier, got: TemplateEnd.\"\n");
}

#[test]
fn shouldPass_whenRowHasActionSequenceWithTwoActions()
{
    let transitionTable = r#"
#include <boost/msm/back/state_machine.hpp>
#include <boost/msm/front/state_machine_def.hpp>
#include <boost/msm/front/functor_row.hpp>
#include <iostream>

struct State1 : public boost::msm::front::state<> {};
struct State2 : public boost::msm::front::state<> {};
struct Event {};

struct Action1
{
    template <class Event, class Fsm, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&)
    {
        std::cout << "performing Action1\n";
    }
};

struct Action2
{
    template <class Event, class Fsm, class Source, class Target>
    void operator()(const Event&, Fsm&, Source&, Target&)
    {
        std::cout << "performing Action2\n";
    }
};

struct MachineDef : public boost::msm::front::state_machine_def<MachineDef>
{
    using initial_state = State1;
    template <typename ...Actions>
    using ActionSequence = boost::msm::front::ActionSequence_<boost::mpl::vector<Actions...>>;
    using None = boost::msm::front::none;
    template <class Source, class Event, class Target, class Action = None, class Guard = None>
    using Row = boost::msm::front::Row<Source, Event, Target, Action, Guard>;

    struct transition_table : boost::mpl::vector<
        Row<State1, Event, State2, ActionSequence<Action1, Action2>>
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
