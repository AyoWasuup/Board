use bevy::prelude::*;

#[derive(Component)]
pub struct GameOverText;

#[derive(Component)]
pub struct GameOverButton;

pub fn setup_gameover(mut commands: Commands, window: Query<&Window>){
    let window = window.single();

    let win_width = window.resolution.width();
    let win_height = window.resolution.height();

    let text = "GAME\nOVER";
    commands.spawn((
        TextBundle::from_section(
            text,
            TextStyle {
                color: Color::RED,
                font_size: 150.0,
                ..default()
            },
        )

        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            align_self: AlignSelf::Center,
            left: Val::Px(90.0),
            bottom: Val::Px(60.0),
            ..default()
        }),
        GameOverText,
    ));

    commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center, 
            bottom: Val::Px(60.0),
            ..default()
        },
        ..default()
    },GameOverButton))
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(300.0),
                height: Val::Px(50.0),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            background_color: Color::BLACK.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "INSERT COIN",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
    });
}

// TODO:
//pub fn setup_main_menu(mut commands: Commands, mut all_en: Query<Entity>) {
//    for entity in &mut all_en {
//        commands.entity(entity).despawn();
//    }
//
//    let text = "board?";
//    commands.spawn()
//}

pub fn end_gameover(mut gameovertext: Query<Entity, With<GameOverText>>, mut commands: Commands){
    for i in &mut gameovertext {
        commands.entity(i).despawn();
    }
}
