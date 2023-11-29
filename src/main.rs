use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_editor_pls::prelude::*;

fn main() {
    App::new()
			     .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(EditorPlugin::default())
        .run();
}
