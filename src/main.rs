use bevy::{prelude::*, math::vec3};
use bimber_animations::{
    asset::Animation,
    component::{AnimatedSpriteBundle, AnimId, AnimatedState, Animated},
    AnimationPlugin,
};

fn spawn_test(mut commands: Commands, server: Res<AssetServer>) {
    let grenadier: Handle<Animation> = server.load("Soldiers/Grenadier-Class.png");
    let grenadier_atlas: Handle<TextureAtlas> = server.load("Soldiers/Grenadier-Class.png#atlas");
    
    let leader: Handle<Animation> = server.load("Soldiers/SquadLeader.png");
    let leader_atlas: Handle<TextureAtlas> = server.load("Soldiers/SquadLeader.png#atlas");
    
    for i in -2..2 {
        commands.spawn(AnimatedSpriteBundle {
            sprite_sheet: SpriteSheetBundle {
                texture_atlas: grenadier_atlas.clone(),
                transform: Transform::from_translation(vec3(i as f32 * 15.0, 0.0, 0.0)),
                ..default()
            },
            animated: AnimatedState {
                curr_state : "idle".id(),
            },
            handle: grenadier.clone(),
            ..default()
        });
    }

    commands.spawn(AnimatedSpriteBundle {
        sprite_sheet: SpriteSheetBundle {
            texture_atlas: leader_atlas.clone(),
            transform: Transform::from_translation(vec3(30.0, 0.0, 0.0)),
            ..default()
        },
        animated: AnimatedState {
            curr_state : "idle".id(),
        },
        handle: leader.clone(),
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
    if input.just_pressed(KeyCode::W) {
        for mut state in query.iter_mut() {
            state.next = Some("walk".id());
        }
    }
    if input.just_pressed(KeyCode::C) {
        for mut state in query.iter_mut() {
            state.next = Some("crawl".id());
        }
    }
    if input.just_pressed(KeyCode::F) {
        for mut state in query.iter_mut() {
            state.next = Some("fire".id());
        }
    }
    if input.just_pressed(KeyCode::H) {
        for mut state in query.iter_mut() {
            state.next = Some("hit".id());
        }
    }
    if input.just_pressed(KeyCode::D) {
        for mut state in query.iter_mut() {
            state.next = Some("death".id());
        }
    }
    if input.just_pressed(KeyCode::T) {
        for mut state in query.iter_mut() {
            state.next = Some("throw".id());
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
