use bevy::{
    prelude::*,
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    window::CursorMoved,
};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(text_update_system)
        .add_system(text_color_system)

        .add_system(print_mouse_events_system)
        .add_system(change_text_system)
        
        .run();
}



#[derive(Component)]
struct MBI_text;

#[derive(Component)]
struct MouseXYtext;

#[derive(Component)]
struct ColorText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
    
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "MBI: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 20.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        //.insert(FpsText);
        .insert(MBI_text);


    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    
                    TextSection {
                        value: "X = ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::RED,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 30.0,
                            color: Color::ORANGE_RED,
                        },
                    },
                    TextSection {
                        value: "\n   Y = ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::YELLOW,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 30.0,
                            color: Color::GREEN,
                        },
                    },
                    
                ],
                alignment: Default::default(),
            },
            ..Default::default()
        })
        .insert(MouseXYtext);
}


// ..................................................... SYSTEMS ..........................................................................
 

fn text_update_system(mut mouse_button_input_events: EventReader<MouseButtonInput>, mut query: Query<&mut Text, With<MBI_text>>) {
    for mut text in query.iter_mut() {

        for event in mouse_button_input_events.iter() {
            //text.sections[1].value = format!("{:.2}", event);
            text.sections[1].value = format!("{:?}", event);


        
            }
        }
    
}



fn change_text_system(
    //time: Res<Time>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut query: Query<&mut Text, With<MouseXYtext>>) {
    for mut text in query.iter_mut() {

        for event in cursor_moved_events.iter() {
            text.sections[1].value = format!("{:?}", event.position[0]);
            text.sections[3].value = format!("{:?}", event.position[1]);
        }

        
    }
}



fn text_color_system(time: Res<Time>, mut query: Query<&mut Text, With<ColorText>>) {
    for mut text in query.iter_mut() {
        let seconds = time.seconds_since_startup() as f32;
        // We used the `Text::with_section` helper method, but it is still just a `Text`,
        // so to update it, we are still updating the one and only section
        text.sections[0].style.color = Color::Rgba {
            red: (1.25 * seconds).sin() / 2.0 + 0.5,
            green: (0.75 * seconds).sin() / 2.0 + 0.5,
            blue: (0.50 * seconds).sin() / 2.0 + 0.5,
            alpha: 1.0,
        };
    }
}

// ----------------- MOUSE

/// This system prints out all mouse events as they come in
fn print_mouse_events_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    for event in mouse_button_input_events.iter() {
        info!("{:?}", event);
    }

    for event in mouse_motion_events.iter() {
        info!("{:?}", event);
    }

    for event in cursor_moved_events.iter() {
        info!("{:?}", event);
    }

    for event in mouse_wheel_events.iter() {
        info!("{:?}", event);
    }
}