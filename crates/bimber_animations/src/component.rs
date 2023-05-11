use bevy::{prelude::*, ecs::query::WorldQuery};

use crate::{asset::{AnimMeta, AnimId}, hash};

use super::asset::Animation;

#[derive(Debug, Component)]
pub struct AnimatedData {
    pub curr_idx: usize,
    pub timer: Timer,
    pub meta: AnimMeta,
}

#[derive(Debug, Component, Default)]
pub struct AnimatedCurr {
    pub id: AnimId,
}

#[derive(Debug, Component, Default)]
pub struct AnimatedQueued {
    pub id: Option<AnimId>,
}

#[derive(Debug, WorldQuery)]
#[world_query(mutable)]
pub struct Animated {
    pub curr: &'static mut AnimatedCurr,
    pub queued: &'static mut AnimatedQueued,
}

impl<'w> AnimatedItem<'w> {
    pub fn change_now(&mut self, id : impl ToAnimId) {
        self.curr.id = id.id();
    }
    pub fn change_if_new(&mut self, id: impl ToAnimId) {
        if self.curr.id != id.id() {
            self.curr.id = id.id();
        }
    }
    pub fn queue(&mut self, id: impl ToAnimId) {
        self.queued.id = Some(id.id());
    }
    pub fn pop_queue(&mut self) -> Option<AnimId> {
        self.queued.id.take()
    }
}

pub trait ToAnimStr {
    fn to_str(&self) -> &str;
}

impl<'a> ToAnimStr for &'a str {
    fn to_str(&self) -> &str {
        self
    }
}

pub trait ToAnimId {
    fn id(&self) -> AnimId;
}

impl ToAnimId for AnimId {
    fn id(&self) -> AnimId {
        *self
    }
}

impl<T : ToAnimStr> ToAnimId for T {
    fn id(&self) -> AnimId {
        hash::hash(self.to_str())
    }
}


#[derive(Default, Bundle)]
pub struct AnimatedSpriteBundle {
    pub animation_state: AnimatedData, 
    pub curr: AnimatedCurr,
    pub queued: AnimatedQueued,
    pub sprite_sheet: SpriteSheetBundle,
    pub handle: Handle<Animation>,
}

impl Default for AnimatedData {
    fn default() -> Self {
        Self {
            curr_idx: 0,
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            meta: default(),
        }
    }
}
