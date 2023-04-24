use bevy::prelude::*;

use crate::{asset::AnimMeta, hash};

use super::asset::Animation;

#[derive(Debug, Component)]
pub struct AnimationState {
    pub curr_idx: usize,
    pub timer: Timer,
    pub is_paused: bool,
    pub meta: AnimMeta
}

#[derive(Debug, Component, Default)]
pub struct Animated {
    pub curr_state: usize,
}

impl Animated {
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
    pub animation_state: AnimationState, 
    pub animated: Animated,
    pub sprite_sheet: SpriteSheetBundle,
    pub handle: Handle<Animation>,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            curr_idx: 0,
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            meta: default(),
            is_paused: false,
        }
    }
}
