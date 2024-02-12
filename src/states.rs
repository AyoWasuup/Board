use bevy::{prelude::*, ecs::query::WorldQuery};
use crate::{player::{Player, MidairTimer, PlayAnimation}, global::{Ground, self}, floor_items::FloorItem};

#[derive(Component)]
pub struct GameOver;

#[derive(Component)]
pub struct GameOverText;

#[derive(Component)]
pub struct GameOverButton;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    MainMenu,
    #[default]
    InGame,
    Paused,
    GameOver,
}

pub fn setup_gameover(mut commands: Commands, window: Query<&Window>){
    let window = window.single();

    let win_width = window.resolution.width();
    let win_height = window.resolution.height();

    let text = "GAME\nOVER";

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
    }, GameOver
    ))
    // game over text
    .with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "GAME\nOVER",
                TextStyle {
                    color: Color::RED,
                    font_size: 150.0,
                    ..default()
                }
            )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            align_self: AlignSelf::Center,
            left: Val::Px(90.0),
            bottom: Val::Px(60.0),
            ..default()
        }),
        ));})
    // button
    .with_children(|parent| {
        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(300.0),
                height: Val::Px(50.0),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                right: Val::Px(140.0),
                bottom: Val::Px(60.0),
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            background_color: Color::BLACK.into(),
            ..default()
        }, GameOverButton
        ))
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

pub fn end_gameover(mut gameovertext: Query<Entity, With<GameOver>>, mut commands: Commands){
    for i in &mut gameovertext {
        commands.entity(i).despawn_recursive();
    }
}

fn update_gameover(
    mut gamestate: ResMut<NextState<GameState>>,
    mut gameoverbutton: Query<
        (
            &Interaction,
            &mut BorderColor,
            Option<&GameOverButton>,
        ),
        (Changed<Interaction>, With<Button>, With<GameOverButton>),
    >,
    mut query: Query<Entity, With<global::GameComponent>>,
    mut commands: Commands,
){
    for (interaction, mut border_color, maybe_gameover) in &mut gameoverbutton {
        if !maybe_gameover.is_some() {
            break;
        }

        match *interaction {
            Interaction::Pressed => {
                for i in &mut query {
                    commands.entity(i).despawn_recursive();
                }    
                gamestate.set(GameState::InGame);
            }
            Interaction::Hovered => {
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                border_color.0 = Color::BLACK;
            }
        }
    }    
}

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) { 
        app.add_systems(OnEnter(GameState::GameOver), setup_gameover)
        .add_systems(Update, (update_gameover).run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), end_gameover);
    }
}

