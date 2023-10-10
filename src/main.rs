use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::window::WindowResolution;

pub use backgrounds::*;
pub use bird::*;
pub use clouds::*;
pub use main_menu::*;
pub use physics::*;
pub use pipes::*;
pub use save::*;
pub use score::*;
pub use scroll::*;

use crate::audio::AudioPlugin;
use crate::death_menu::DeathMenuPlugin;

mod audio;
mod backgrounds;
mod bird;
mod clouds;
mod death_menu;
mod debug;
mod main_menu;
mod physics;
mod pipes;
mod save;
mod score;
mod scroll;

#[derive(States, Eq, PartialEq, Debug, Clone, Hash, Default)]
enum GameState {
    #[default]
    MainMenu,
    Playing,
    Dead,
}

#[derive(Component)]
struct InPlayState;

fn init_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgba(
                95. / 255.,
                205. / 255.,
                255. / 255.,
                255. / 255.,
            )),
        },
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Rusty Bird".to_string(),
                        resolution: WindowResolution::new(800., 600.),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
            MainMenuPlugin,
            DeathMenuPlugin,
            PipesPlugin,
            BirdPlugin,
            ScorePlugin,
            ScrollPlugin,
            CloudsPlugin,
            PhysicsPlugin,
            AudioPlugin,
            BackgroundsPlugin,
            SavePlugin,
            // DebugPlugin,
        ))
        .add_state::<GameState>()
        .add_systems(Startup, init_camera)
        .run();
}
