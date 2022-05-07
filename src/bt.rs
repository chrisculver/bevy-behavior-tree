use bevy::{
    ecs::{component::Component, query::WorldQuery, world::Mut},
    prelude::Query,
};

use std::fmt::Debug;
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

#[derive(WorldQuery, Component)]
#[world_query(mutable, derive(Debug))]
pub struct BehaviorTree<'w, T: Tickable + Component> {
    pub root: &'w mut T,
}

// We should tick all trees!
// We can add an option time/frame delay to not tick them every frame
pub fn tick_behavior_trees<T: Tickable + Component>(mut bt_query: Query<&mut BehaviorTree<'static, T>>) {
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

#[derive(Component)]
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

pub trait Tickable: Send + Sync {
    fn tick(&mut self) -> Status;
}
