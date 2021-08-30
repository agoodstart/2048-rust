use bevy::prelude::*;
use bevy::render::pass::ClearColor;


struct Grid {
    block: Handle<ColorMaterial>,
}

fn main() {
    App::build()    
        .insert_resource(ClearColor(Color::hex("FAF8EE").unwrap()))
        .insert_resource(WindowDescriptor {
            title: "I am a window!!!".to_string(),
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}