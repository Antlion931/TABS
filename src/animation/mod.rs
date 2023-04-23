pub mod asset;
pub mod component;
pub mod system;

use bevy::prelude::*;

use self::{
    component::UntypedAnimState,
    system::{advance_anims, on_change_anim_state, on_load_anim}, asset::{AnimationLoader, Animation},
};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<AnimationLoader>()
            .add_asset::<Animation>()
            .add_system(advance_anims)
            .add_system(on_change_anim_state::<UntypedAnimState>)
            .add_system(on_load_anim::<UntypedAnimState>);
    }
}
