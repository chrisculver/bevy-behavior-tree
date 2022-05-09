use bevy::ecs::component::Component;
use bevy::prelude::Query;
use std::sync::{Arc, Mutex};

// TODO: ATM this will lock up threads while waiting for the BT to be made available...
// Could run ALL the AI on it's own thread?
// How to avoid MuteX?

#[derive(PartialEq, Eq)]
pub enum Status {
    Success,
    Failure,
    Running,
}

pub trait Node {
    fn tick(&mut self) -> Status;
}

#[derive(Component)]
pub struct BehaviorTree {
    root: Arc<Mutex<dyn Node + Send + Sync>>,
}

impl BehaviorTree {
    pub fn new(new_root: Arc<Mutex<dyn Node + Send + Sync>>) -> BehaviorTree {
        BehaviorTree {
            root: new_root
        }
    }
}

#[derive(Component)]
pub struct Action {
    pub func: Arc<dyn Fn() -> Status + Send + Sync>,
}

impl Node for Action {
    fn tick(&mut self) -> Status {
        return (self.func)();
    }
}

#[macro_export]
macro_rules! action {
    ( $e:expr ) => {
        Arc::new(Mutex::new(Action { func: Arc::new($e) }))
    };
}

#[derive(Component)]
pub struct Sequence {
    pub children: Vec<Arc<Mutex<dyn Node + Send + Sync>>>,
    pub active: usize,
}

// TODO: Test this is the correct behavior.
impl Node for Sequence {
    fn tick(&mut self) -> Status {
        let child_status = self.children[self.active].lock().unwrap().tick();
        if child_status == Status::Success {
            // if a child succeeds and they are not the last child go to next
            if self.active + 1 < self.children.len() {
                self.active += 1;
                return Status::Running;
            } else {
                // all of the children succeeded so this node succeeds
                return Status::Success;
            }
        } else if child_status == Status::Failure {
            self.active = 0;
        }
        return child_status;
    }
}

#[macro_export]
macro_rules! sequence
{
	( $( $e:expr ),* ) => {
		Arc::new(Mutex::new(Sequence {
            children: vec![$( $e ),*],
            active: 0
        }))
	};
}


pub fn test_run_bts(mut bt_query: Query<&mut BehaviorTree>) {
    for bt in bt_query.iter_mut() {
        bt.root.lock().unwrap().tick();
    }
}
