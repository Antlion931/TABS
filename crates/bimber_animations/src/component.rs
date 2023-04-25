use bevy::prelude::*;

use crate::{asset::AnimMeta, hash};

use super::asset::Animation;

#[derive(Debug, Component)]
pub struct Animated {
    pub curr_idx: usize,
    pub timer: Timer,
    pub meta: AnimMeta,
    pub next: Option<usize>
}

#[derive(Debug, Component, Default)]
pub struct AnimatedState {
    pub curr_state: usize,
}

impl AnimatedState {
    pub fn change_state(&mut self, state: impl AnimId) {
        self.curr_state = state.id();
    }
}

pub trait AnimId {
    fn to_str(&self) -> &str;
    fn id(&self) -> usize {
        hash::hash(self.to_str())
    }
}

impl<'a> AnimId for &'a str {
    fn to_str(&self) -> &str {
        self
    }
}

#[derive(Default, Bundle)]
pub struct AnimatedSpriteBundle {
    pub animation_state: Animated, 
    pub animated: AnimatedState,
    pub sprite_sheet: SpriteSheetBundle,
    pub handle: Handle<Animation>,
}

impl Default for Animated {
    fn default() -> Self {
        Self {
            curr_idx: 0,
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            meta: default(),
            next: None,
        }
    }
}
