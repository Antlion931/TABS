use std::time::Duration;

use bevy::prelude::*;

use crate::{
    asset::{AnimId, AnimMode},
    component::{Animated, AnimatedCurr, AnimatedData},
};

use super::asset::Animation;

pub struct AnimFinishedEvent {
    pub entity: Entity,
    pub fin: AnimId,
}

pub fn advance_anims(
    mut query: Query<(Entity, &mut AnimatedData, Animated, &mut TextureAtlasSprite)>,
    mut writer: EventWriter<AnimFinishedEvent>,
    time: Res<Time>,
) {
    for (ent, mut state, mut anim, mut sprite) in query.iter_mut() {
        state.timer.tick(time.delta());

        state.curr_idx += state.timer.times_finished_this_tick() as usize;
        if state.curr_idx >= state.meta.len {
            state.curr_idx %= state.meta.len;

            if let Some(next) = anim.pop_queue() {
                if next != anim.curr.id {
                    anim.change_now(next);
                }
                if state.meta.mode == AnimMode::Repeating {
                    anim.queue(anim.curr.id)
                }
            } else {
                // Only sends when there is no next animation. Can be changed when a usecase is found
                writer.send(AnimFinishedEvent {
                    entity: ent,
                    fin: anim.curr.id,
                });

                state.timer.pause();
                state.curr_idx = state.meta.len - 1;
            }
        }
        sprite.index = state.meta.start_idx + state.curr_idx;
    }
}

pub fn on_change_anim(
    mut query: Query<(&mut AnimatedData, Animated, &Handle<Animation>), Changed<AnimatedCurr>>,
    anims: Res<Assets<Animation>>,
) {
    for (mut state, mut anim, handle) in query.iter_mut() {
        debug!("Changing animations");

        let anim_data = match anims.get(handle) {
            Some(a) => a,
            None => {
                warn!("Animation not loaded yet");
                continue;
            }
        };

        let next_anim_id = anim.curr.id;

        let next_anim = match anim_data.get_with_id(next_anim_id) {
            Some(a) => a,
            None => {
                warn!("Animation not found");
                continue;
            }
        };

        state.meta = next_anim.clone();
        state.curr_idx %= state.meta.len;

        match state.meta.mode {
            AnimMode::Repeating => anim.queue(next_anim_id),
            AnimMode::Once => {
                anim.pop_queue();
            }
        }

        state
            .timer
            .set_duration(Duration::from_secs_f32(next_anim.frame_time));
        state.timer.unpause();
        state.timer.reset();

        debug!("Changed anim to: {:?}", next_anim);
    }
}

pub fn on_load_anim(
    mut query: Query<(Animated, &Handle<Animation>)>,
    mut events: EventReader<AssetEvent<Animation>>,
) {
    for event in events.iter() {
        debug!("Triggered reload");
        if let AssetEvent::Modified { handle } = event {
            for (mut anim, anim_handle) in query.iter_mut() {
                if anim_handle == handle {
                    debug!("Updated anim");
                    anim.curr.set_changed();
                }
            }
        }
    }
}
