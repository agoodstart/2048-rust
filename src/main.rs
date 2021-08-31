use bevy::prelude::*;
use bevy::render::pass::ClearColor;


// struct Grid {
//     block: Handle<ColorMaterial>,
// }

const _GRID_ROW: u32 = 4;
const _GRID_COLUMN: u32 = 4;

fn main() {
    App::build()    
        .insert_resource(ClearColor(Color::hex("FAF8EE").unwrap()))
        .insert_resource(WindowDescriptor {
            title: "I am a window!!!".to_string(),
            width: 400.0,
            height: 600.0,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>) {

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let mut window = windows.get_primary_mut().unwrap();

    window.set_position(IVec2::new(80, 50));

    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        material: materials.add(Color::NONE.into()),
        ..Default::default()
    })
    // Bottom Wrapper
    .with_children(|parent| {
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(70.0)),
                    // border: Rect::all(Val::Px(2.0)),
                    ..Default::default()
                },
                material: materials.add(Color::rgb(0.65, 0.65, 0.65).into()),
                ..Default::default()
            });

        // Top Wrapper
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                    // border: Rect::all(Val::Px(2.0)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                material: materials.add(Color::rgb(0.0, 0.0, 0.28).into()),
                ..Default::default()
            })
        // left side
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },

                ..Default::default()
            })
            // Text
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "2048",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::BLACK,
                        },
                    Default::default()
                ),
                    ..Default::default()
                });
            });
        });
    });
}