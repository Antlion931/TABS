use bevy::{
    math::{vec2, vec3},
    prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, log::{LogPlugin, Level},
};
use bimber_animations::{
    asset::Animation,
    component::{ToAnimId, AnimatedData, AnimatedSpriteBundle, AnimatedCurr, Animated, ToAnimStr},
    AnimationPlugin, system::AnimFinishedEvent, AnimSet,
};

#[derive(Debug, Component)]
struct Leader;

/// Spawning example using the [Animation] asset
fn spawn_test(mut commands: Commands, server: Res<AssetServer>) {
    // Loading animation data with
    // Note the .png and #atlas at end
    let grenadier: Handle<Animation> = server.load("Soldiers/Grenadier-Class.png");
    let grenadier_atlas: Handle<TextureAtlas> = server.load("Soldiers/Grenadier-Class.png#atlas");

    let leader: Handle<Animation> = server.load("Soldiers/SquadLeader.png");
    let leader_atlas: Handle<TextureAtlas> = server.load("Soldiers/SquadLeader.png#atlas");

    for i in -2..=2 {
        for j in 0..1 {
            commands.spawn(AnimatedSpriteBundle {
                sprite_sheet: SpriteSheetBundle {
                    texture_atlas: grenadier_atlas.clone(),
                    transform: Transform::from_translation(vec3(i as f32 * 15.0, j as f32 * 15.0, 0.0)),
                    ..default()
                },
                curr: AnimatedCurr {
                    id: "idle".id(),
                },
                handle: grenadier.clone(),
                ..default()
            });
        }
    }

    commands
        .spawn(AnimatedSpriteBundle {
            sprite_sheet: SpriteSheetBundle {
                texture_atlas: leader_atlas,
                transform: Transform::from_translation(vec3(30.0, 0.0, 0.0)),
                ..default()
            },
            curr: AnimatedCurr {
                id: "idle".id(),
            },
            handle: leader,
            ..default()
        })
        .insert(Leader);

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.1,
            ..default()
        },
        ..default()
    });
}

/// Animation with basic controller. We are querying for [Animated]
fn movement(
    mut query: Query<(&mut Transform, &mut TextureAtlasSprite, Animated), With<Leader>>,
    input: Res<Input<KeyCode>>,
) {
    for (mut trans, mut sprite, mut anim) in query.iter_mut() {
        let mut mov = vec2(0., 0.);

        if input.pressed(KeyCode::W) {
            mov += Vec2::Y;
        }
        if input.pressed(KeyCode::S) {
            mov += Vec2::NEG_Y;
        }
        if input.pressed(KeyCode::D) {
            mov += Vec2::X;
            sprite.flip_x = false;
        }
        if input.pressed(KeyCode::A) {
            mov += Vec2::NEG_X;
            sprite.flip_x = true;
        }

        let norm = mov.normalize_or_zero();
        // Easily check which animation is playing, note the '.id()'
        if norm == Vec2::ZERO && anim.curr.id == "walk".id() {
            // Changes animation
            anim.change_now("idle");
        } else if norm != Vec2::ZERO {
            // Changes animation only if its not currently playing
            anim.change_if_new("walk");
        }

        trans.translation += norm.extend(0.);
    }
}

/// Animations can be strongly typed to help avoid spelling errors
/// For this implement [ToAnimStr] or [ToAnimId]
enum SoldierAnimation {
    _Crawl, Fire, Hit, Death, Throw, _Walk, _Idle
}
impl ToAnimStr for SoldierAnimation {
    fn to_str(&self) -> &str {
        match self {
            SoldierAnimation::_Crawl => "crawl",
            SoldierAnimation::Fire => "fire",
            SoldierAnimation::Hit => "hit",
            SoldierAnimation::Death => "death",
            SoldierAnimation::Throw => "throw",
            SoldierAnimation::_Walk => "walk",
            SoldierAnimation::_Idle => "idle",
        }
    }
}

/// Animation can be queued to run after the current animation is finished
fn special_input(mut query: Query<Animated>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::C) {
        for mut anim in query.iter_mut() {
            // Queueing
            anim.queue("crawl");
        }
    }
    if input.just_pressed(KeyCode::F) {
        for mut anim in query.iter_mut() {
            // Queuing with typed animation
            anim.queue(SoldierAnimation::Fire);
        }
    }
    if input.just_pressed(KeyCode::H) {
        for mut anim in query.iter_mut() {
            anim.queue(SoldierAnimation::Hit);
        }
    }
    if input.just_pressed(KeyCode::K) {
        for mut anim in query.iter_mut() {
            anim.queue(SoldierAnimation::Death);
        }
    }
    if input.just_pressed(KeyCode::T) {
        for mut anim in query.iter_mut() {
            anim.queue(SoldierAnimation::Throw);
        }
    }
}

/// When animation without any more queued, it emits [AnimFinishedEvent].
/// This can be used to make transitions
fn when_finished(mut query: Query<Animated>, mut reader: EventReader<AnimFinishedEvent>) {
    for event in reader.iter() {
        if event.fin != "death".id() {
            if let Ok(mut anim) = query.get_mut(event.entity) {
                anim.change_now("walk");
            }
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
                })
                .set(LogPlugin {
                    filter: "info,wgpu=error,bimber_animations=debug".into(),
                    level: Level::DEBUG,
                }),
        )
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(AnimationPlugin)
        .add_startup_system(spawn_test)
        .add_systems((special_input,movement,when_finished).before(AnimSet))
        .run();
}
