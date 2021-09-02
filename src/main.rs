use bevy::prelude::*;
use bevy::render::pass::ClearColor;

// mod components;

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
        .add_plugins(DefaultPlugins)
        .init_resource::<ButtonMaterials>()
        .add_startup_system(setup.system())
        .add_system(button_system.system())
        .run();
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
    mut windows: ResMut<Windows>) {

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let window = windows.get_primary_mut().unwrap();

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
    
    .with_children(|parent| {

    // Bottom Wrapper
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(70.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    // border: Rect::all(Val::Px(2.0)),
                    ..Default::default()
                },
                material: materials.add(Color::rgb(0.65, 0.65, 0.65).into()),
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(NodeBundle {

                        ..Default::default()
                    });
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

            .with_children(|parent| {

            // left side top wrapper
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

            .with_children(|parent| {

                // "2048" text block
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(70.0), Val::Percent(70.0)),
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: materials.add(Color::hex("eccc5f").unwrap().into()),
                    ..Default::default()
                })

                .with_children(|parent| {
                    
                    // "2048" text
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            margin: Rect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        text: Text::with_section(
                            "2048",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 50.0,
                                color: Color::WHITE,
                            },
                        Default::default()
                    ),
                        ..Default::default()
                    });
                });
            });

            // right side top wrapper
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
            .with_children(|parent| {
                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Restart",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
            });
        });
    });
}