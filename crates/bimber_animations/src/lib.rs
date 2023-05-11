pub mod asset;
pub mod component;
pub mod system;
pub mod hash;

use bevy::prelude::*;
use system::AnimFinishedEvent;

use self::{
    system::{advance_anims, on_change_anim, on_load_anim}, asset::{AnimationLoader, Animation},
};

pub struct AnimationPlugin;

#[derive(SystemSet, Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnimSet;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<AnimationLoader>()
            .add_event::<AnimFinishedEvent>()
            .add_asset::<Animation>()
            .add_systems((on_load_anim, advance_anims, on_change_anim).chain().in_set(AnimSet));
    }
}
