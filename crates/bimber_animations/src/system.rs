use std::time::Duration;

use bevy::prelude::*;

use crate::asset::AnimMode;

use super::{asset::Animation, component::{Animated, AnimState}};

pub fn advance_anims(mut query : Query<(&mut Animated, &mut TextureAtlasSprite)>, time: Res<Time>) {
    for (mut animated, mut sprite) in query.iter_mut() {
        animated.timer.tick(time.delta());
        match animated.meta.mode {
            AnimMode::Repeating => {
                animated.curr_idx = (animated.curr_idx + animated.timer.times_finished_this_tick() as usize) % animated.meta.len;
            },
            AnimMode::Once => {
                animated.curr_idx += animated.timer.times_finished_this_tick() as usize;
                if animated.curr_idx >= animated.meta.len - 1 {
                    animated.curr_idx = animated.meta.len - 1;
                    animated.is_paused = true;
                }
            },
        }

        sprite.index = animated.meta.start_idx + animated.curr_idx;
    }
}

pub fn on_change_anim_state<A : AnimState>(mut query : Query<(&mut Animated, &A), Changed<A>>, anims: Res<Assets<Animation>>) {
    for (mut animated, a) in query.iter_mut() {
        info!("Changing animations {}", a.to_str());
        
        let anim_data = match anims.get(&animated.animation) {
            Some(a) => a,
            None => {
                warn!("Animation not loaded yet {}", a.to_str());
                continue;
            },
        };

        let next_anim = match anim_data.map.get(a.to_str()) {
            Some(a) => a,
            None =>  {
                warn!("Animation not found {}", a.to_str());
                continue;
            },
        };

        animated.curr_idx = 0;
        animated.meta = next_anim.clone();
        animated.is_paused = false;

        animated.timer.set_duration(Duration::from_secs_f32(next_anim.frame_time));
        animated.timer.reset();
    }
}

pub fn on_load_anim<A : AnimState>(mut query : Query<(&Animated, &mut A)>, mut events: EventReader<AssetEvent<Animation>>) {
    for event in events.iter() {
        info!("Triggered reload");
        if let AssetEvent::Modified { handle } = event {
            for (animated, mut state) in query.iter_mut() {
                if &animated.animation == handle {
                    info!("Updated anim");
                    state.set_changed();
                }
            } 
        }
    }
}
