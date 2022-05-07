use bevy::prelude::{Component, Query};

use std::sync::Arc;

#[derive(PartialEq, Eq)]
pub enum Status {
    NeverRun,
    Success,
    Failure,
    Running,
}

//  We want to build a behavior tree
//
//             Root
//               v
//            Sequence
//        v -------- v ------- v
//      Action1    Action2   Selector
//                           v ----------v
//                        Action3      Action4

// and arbitrarily complicated...

// The entity is the AI it's attached to.
// The tree itself is a component
// Ticking the tree is a system

// So I want to do something like
// commands.insert(BasicBot)
//         .insert(BasicBotBehaviorTree);
// app.add_system(tick_behavior_tree);

#[derive(Component)]
pub struct BehaviorTree {
    pub root: Sequence,
}

pub fn tick_behavior_trees(mut bt_query: Query<&mut BehaviorTree>) {
    for mut bt in bt_query.iter_mut() {
        bt.root.tick();
    }
}

pub struct Action {
    pub func: Arc<dyn Fn() -> Status + Send + Sync>,
}

impl Tickable for Action {
    fn tick(&mut self) -> Status {
        return (self.func)();
    }
}

pub struct Sequence {
    // Of course sequences can have non Actions as children...
    // Probalby needs a Vec of T where T is tickable?
    pub children: Vec<Box<Action>>,
    pub active: usize,
}

impl Tickable for Sequence {
    fn tick(&mut self) -> Status {
        let child_status = self.children[self.active].tick();
        if child_status == Status::Success {
            if self.active + 1 < self.children.len() {
                self.active += 1;
            } else {
                return Status::Success;
            }
        }

        return child_status;
    }
}

//impl Tick for Action {
//    fn tick(&self) -> Status {
//        return (self.func)()
//    }
//}

trait Tickable: Send + Sync {
    fn tick(&mut self) -> Status;
}

//impl<T> Tick for T
//where
//    T: Component + Send + Sync,
//{
//    fn tick(&mut self) -> Status {
//        Status::Running
//    }
//}
