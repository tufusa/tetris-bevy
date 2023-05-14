use bevy::prelude::*;
use rand::Rng;
// use rand::prelude::*;

const UNIT_WIDTH: u32 = 40;
const UNIT_HEIGHT: u32 = 40;

const X_LENGTH: u32 = 10;
const Y_LENGTH: u32 = 18;

const SCREEN_WIDTH: u32 = UNIT_WIDTH * X_LENGTH;
const SCREEN_HEIGHT: u32 = UNIT_HEIGHT * Y_LENGTH;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Resource)]
struct Materials {
    colors: Vec<Color>,
}

struct Mino {
    patterns: Vec<(i32, i32)>,
    color: Color,
}

#[derive(Resource)]
struct Minos(Vec<Mino>);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Tetris".into(),
                        resolution: (SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..Default::default()
                }),
        )
        .add_startup_system(setup) // startupは複数登録するとまずい
        .add_system(position_transform)
        .add_system(spawn_mino)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(Minos(vec![
        Mino {
            patterns: vec![(0, 0), (0, -1), (0, 1), (0, 2)], // I
            color: Color::hex("84CDEE").unwrap(),
        },
        Mino {
            patterns: vec![(0, 0), (1, 0), (-1, 0), (1, 1)], // L
            color: Color::hex("FFB21B").unwrap(),
        },
        Mino {
            patterns: vec![(0, 0), (1, 0), (-1, 0), (-1, 1)], // J
            color: Color::hex("021496").unwrap(),
        },
        Mino {
            patterns: vec![(0, 0), (0, 1), (1, 0), (-1, 1)], // Z
            color: Color::hex("DE0000").unwrap(),
        },
        Mino {
            patterns: vec![(0, 0), (0, 1), (-1, 0), (1, 1)], // S
            color: Color::hex("88FF55").unwrap(),
        },
        Mino {
            patterns: vec![(0, 0), (0, 1), (1, 0), (1, 1)], // O
            color: Color::hex("F9E909").unwrap(),
        },
        Mino {
            patterns: vec![(0, 0), (-1, 0), (1, 0), (0, 1)], // T
            color: Color::hex("9C0FBF").unwrap(),
        },
    ]))
}

fn spawn_mino(mut commands: Commands, minos: Res<Minos>) {
    let mut rng = rand::thread_rng();
    let mino_index : usize = rng.gen::<usize>() % minos.0.len();
    let next_mino = &minos.0[mino_index];

    let pos_x = X_LENGTH as i32 / 2;
    let pos_y = Y_LENGTH as i32 / 2;

    next_mino.patterns.iter().for_each(|(r_x, r_y)| {
        spawn_block(
            &mut commands,
            next_mino.color,
            Position {
                x: pos_x + r_x,
                y: pos_y + r_y,
            },
        )
    })
}

fn spawn_block(commands: &mut Commands, color: Color, position: Position) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(position);
}

fn position_transform(mut position_query: Query<(&Position, &mut Transform, &mut Sprite)>) {
    let origin_x = -(SCREEN_WIDTH as i32) / 2 + UNIT_WIDTH as i32 / 2;
    let origin_y = -(SCREEN_HEIGHT as i32) / 2 + UNIT_HEIGHT as i32 / 2;
    position_query
        .iter_mut()
        .for_each(|(pos, mut transform, mut _sprite)| {
            transform.translation = Vec3 {
                x: (origin_x + pos.x as i32 * UNIT_WIDTH as i32) as f32,
                y: (origin_y + pos.y as i32 * UNIT_HEIGHT as i32) as f32,
                z: 0.,
            };
            transform.scale = Vec3 {
                x: UNIT_WIDTH as f32,
                y: UNIT_HEIGHT as f32,
                z: 0.,
            };
        });
}
