use std::time::Duration;

use bevy::prelude::*;

use crate::{asset::AnimMode, component::{AnimationState, Animated}};

use super::asset::Animation;

pub fn advance_anims(mut query : Query<(&mut AnimationState, &mut Animated, &mut TextureAtlasSprite)>, time: Res<Time>) {
    for (mut state, mut anim, mut sprite) in query.iter_mut() {
        state.timer.tick(time.delta());
        match state.meta.mode {
            AnimMode::Repeating => {
                state.curr_idx = (state.curr_idx + state.timer.times_finished_this_tick() as usize) % state.meta.len;
            },
            AnimMode::Once(next) => {
                state.curr_idx += state.timer.times_finished_this_tick() as usize;
                if state.curr_idx >= state.meta.len - 1 {
                    state.curr_idx = state.meta.len - 1;
                    state.is_paused = true;
                    if let Some(next) = next {
                        anim.curr_state = next;
                    }
                }
            },
        }

        sprite.index = state.meta.start_idx + state.curr_idx;
    }
}

pub fn on_change_anim_state(mut query : Query<(&mut AnimationState, &Animated, &Handle<Animation>), Changed<Animated>>, anims: Res<Assets<Animation>>) {
    for (mut state, anim, handle) in query.iter_mut() {
        info!("Changing animations");
        
        let anim_data = match anims.get(handle) {
            Some(a) => a,
            None => {
                warn!("Animation not loaded yet");
                continue;
            },
        };

        let next_anim = match anim_data.get_with_hash(anim.curr_state) {
            Some(a) => a,
            None =>  {
                warn!("Animation not found");
                continue;
            },
        };

        state.curr_idx = 0;
        state.meta = next_anim.clone();
        state.is_paused = false;

        state.timer.set_duration(Duration::from_secs_f32(next_anim.frame_time));
        state.timer.reset();
    }
}

pub fn on_load_anim(mut query : Query<(&mut Animated, &Handle<Animation>)>, mut events: EventReader<AssetEvent<Animation>>) {
    for event in events.iter() {
        info!("Triggered reload");
        if let AssetEvent::Modified { handle } = event {
            for (mut anim, anim_handle) in query.iter_mut() {
                if anim_handle == handle {
                    info!("Updated anim");
                    anim.set_changed();
                }
            } 
        }
    }
}
