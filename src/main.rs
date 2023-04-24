use bevy::prelude::*;
use bimber_animations::{
    asset::Animation,
    component::{AnimatedSpriteBundle, AnimId, Animated},
    AnimationPlugin,
};

fn spawn_test(mut commands: Commands, server: Res<AssetServer>) {
    let animation: Handle<Animation> = server.load("Test/test.png");
    let atlas: Handle<TextureAtlas> = server.load("Test/test.png#atlas");

    commands.spawn(AnimatedSpriteBundle {
        sprite_sheet: SpriteSheetBundle {
            texture_atlas: atlas,
            ..default()
        },
        animated: Animated {
            curr_state : "idle".id(),
        },
        handle: animation,
        ..default()
    });

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.1,
            ..default()
        },
        ..default()
    });
}

fn die_on_input(mut query: Query<&mut Animated>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        for mut state in query.iter_mut() {
            state.curr_state = "injure".id();
        }
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }),
        )
        .add_plugin(AnimationPlugin)
        .add_startup_system(spawn_test)
        .add_system(die_on_input)
        .run();
}
