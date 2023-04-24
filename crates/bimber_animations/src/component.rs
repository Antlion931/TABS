use std::time::Duration;

use bevy::prelude::*;

use crate::asset::AnimMeta;

use super::asset::Animation;

#[derive(Debug, Component)]
pub struct Animated {
    pub animation: Handle<Animation>,
    pub curr_idx: usize,
    pub is_paused: bool,
    pub meta: AnimMeta,
    pub timer: Timer,
}

#[derive(Debug, Component)]
pub struct UntypedAnimState(pub &'static str);

#[derive(Default, Bundle)]
pub struct AnimatedSpriteBundle<A : AnimState> {
    pub anim_state: A,
    pub animated: Animated,
    pub sprite_sheet: SpriteSheetBundle,
}

pub trait AnimState : Component {
    fn to_str(&self) -> &str;
}

impl AnimState for UntypedAnimState {
    fn to_str(&self) -> &str {
        self.0
    }
}

impl Default for Animated {
    fn default() -> Self {
        Self {
            animation: Default::default(),
            curr_idx: 0,
            meta: Default::default(),
            timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
            is_paused: true
        }
    }
}
