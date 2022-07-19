use anyhow::{bail, Result};
use cosmo_store::types::event_write::EventWrite;
use cosmo_store_util::aggregate::Aggregate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Clone, Debug)]
pub enum Command {
    AddOrg,
    AddEmployee
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Event {
    OrgAdded,
    EmployeeAdded
}

impl From<Event> for EventWrite<Event, Event> {
    fn from(a : Event) -> Self {
        EventWrite {
            id : Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name : String::from("index_event"),
            data : a,
            metadata: None
        }
    }
}

#[derive(Clone, Debug)]
pub struct State {
    org : i16,
    employee : i16
}

impl State {
    pub const fn init() -> State {
        State {
            org: 0,
            employee: 0
        }
    }
}

#[derive(Clone, Debug)]
pub struct IndexAggregate {
    initial_state : State
}

impl IndexAggregate {
    pub const fn init() -> IndexAggregate {
        IndexAggregate {
            initial_state : State::init()
        }
    }
}

impl Aggregate<State, Command, Event> for IndexAggregate {
    fn init(&self) -> State {
        self.clone().initial_state
    }

    fn apply(&self, state: State, event: &Event) -> State {
        match event {
            Event::OrgAdded => {
                State {
                    org : state.org + 1,
                    ..state
                }
            }
            Event::EmployeeAdded => {
                State {
                    employee : state.employee + 1,
                    ..state
                }
            }
        }
    }

    fn execute(&self, state: &State, command: &Command) -> Result<Vec<Event>> {
        match command {
            Command::AddOrg => {
                if state.org == 100 {
                    bail!("Can't go beyond 100 orgs")
                } else {
                    Ok(vec![Event::OrgAdded])
                }
            }
            Command::AddEmployee => {
                if state.employee == 1000 {
                    bail!("Can't go beyond 1000 employees")
                } else {
                    Ok(vec![Event::EmployeeAdded])
                }
            }
        }
    }
}


