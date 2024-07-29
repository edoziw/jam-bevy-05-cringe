#[cfg(feature = "dev")]
mod dev_tools;
mod game;
mod screen;
mod ui;

use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    prelude::*,
    render::view::visibility::RenderLayers,
};
use bevy_sprite3d::Sprite3dPlugin;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Order new `AppStep` variants by adding them here:
        app.configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        );

        // Spawn the cameras.
        app.add_systems(Startup, spawn_cameras);
        app.add_systems(Update, clone_camera_transforms);

        // Add Bevy plugins.
        app.add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Jam Bevy 05 cringe".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::new(0.3),
                    },
                    ..default()
                }),
            Sprite3dPlugin,
        ));

        // Add other plugins.
        app.add_plugins((game::plugin, screen::plugin, ui::plugin));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum AppSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
struct FollowUiCamera;

fn spawn_cameras(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera0"),
        Camera3dBundle {
            camera: Camera {
                order: 0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        // background camera layer 0
        RenderLayers::from_layers(&[0]),
        FollowUiCamera,
    ));
    commands.spawn((
        Name::new("Camera1"),
        Camera3dBundle {
            camera: Camera {
                order: 1,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        // foreground camera layer 1
        RenderLayers::from_layers(&[1]),
        // Render all UI to this camera.
        // Not strictly necessary since we only use one camera,
        // but if we don't use this component, our UI will disappear as soon
        // as we add another camera. This includes indirect ways of adding cameras like using
        // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        IsDefaultUiCamera,
    ));
}

// camera3dBundle with component IsDefaultUICamera 's transform should be copied to camera3dBundle with component FollowUiCamera
// there is only one of each type of camera so use let Ok form
fn clone_camera_transforms(
    ui_cameras: Query<(Entity, &Transform), With<IsDefaultUiCamera>>,
    mut follow_cameras: Query<&mut Transform, (With<FollowUiCamera>, Without<IsDefaultUiCamera>)>,
) {
    let (_, ui_transform) = ui_cameras.single();
    let mut follow_transform = follow_cameras.single_mut();
    *follow_transform = *ui_transform;
}
