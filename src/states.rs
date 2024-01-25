use bevy::prelude::*;

#[derive(Component)]
pub struct GameOverText;

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
            ..default()
        }),
        GameOverText,
    ));

    println!("{}, {}", win_width, win_width /2.0);
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
