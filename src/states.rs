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
            text: Text {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    value: "Game Over".to_string(),
                    style: TextStyle {
                        alignment: TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                        color: Color::ORANGE,
                        font_size: 150.0,
                    },
                },

            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                align_self: AlignSelf::Center,
                ..default()
            }),
            GameOverText,
        ));
}

pub fn end_gameover(mut gameovertext: Query<Entity, With<GameOverText>>, mut commands: Commands){
    for i in &mut gameovertext {
        commands.entity(i).despawn();
    }
}
