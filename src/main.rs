use bevy::prelude::*;
use rand::Rng;

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
struct Minos(Vec<Mino>);

struct Mino {
    patterns: Vec<(i32, i32)>,
    color: Color,
}

#[derive(Resource)]
struct GameBoard(Vec<Vec<bool>>);

struct NewBlockEvent;

#[derive(Resource)]
struct GameTimer(Timer);

#[derive(Resource)]
struct InputTimer(Timer);

#[derive(Component)]
struct Fix;

#[derive(Component)]
struct Free;

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
        .add_event::<NewBlockEvent>()
        .add_startup_system(setup) // startupは複数登録するとまずい
        .add_system(position_transform)
        .add_system(spawn_mino)
        .add_system(game_timer)
        .add_system(block_fall)
        .add_system(block_horizontal_move)
        .run();
}

fn setup(mut commands: Commands, mut new_block_events_writer: EventWriter<NewBlockEvent>) {
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
    ]));
    commands.insert_resource(GameTimer(Timer::new(
        std::time::Duration::from_millis(400),
        TimerMode::Repeating,
    )));
    commands.insert_resource(InputTimer(Timer::new(
        std::time::Duration::from_millis(50),
        TimerMode::Repeating,
    )));
    commands.insert_resource(GameBoard(vec![vec![false; 25]; 25]));
    new_block_events_writer.send(NewBlockEvent)
}

fn game_timer(
    time: Res<Time>,
    mut game_timer: ResMut<GameTimer>,
    mut input_timer: ResMut<InputTimer>,
) {
    game_timer.0.tick(time.delta());
    input_timer.0.tick(time.delta());
}

fn spawn_mino(
    mut commands: Commands,
    minos: Res<Minos>,
    mut new_block_events_reader: EventReader<NewBlockEvent>,
) {
    if new_block_events_reader.iter().next().is_none() {
        return;
    }

    let mut rng = rand::thread_rng();
    let mino_index: usize = rng.gen::<usize>() % minos.0.len();
    let next_mino = &minos.0[mino_index];

    let pos_x = X_LENGTH as i32 / 2;
    let pos_y = Y_LENGTH as i32;

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

fn block_fall(
    mut commands: Commands,
    timer: ResMut<GameTimer>,
    mut block_query: Query<(Entity, &mut Position, &Free)>,
    mut game_board: ResMut<GameBoard>,
    mut new_block_events_writer: EventWriter<NewBlockEvent>,
) {
    if !timer.0.finished() {
        return;
    }

    let fallable = !block_query.iter().any(|(_, pos, _)| {
        if pos.x as u32 >= X_LENGTH || pos.y as u32 >= Y_LENGTH {
            return false;
        }

        pos.y == 0 || game_board.0[pos.y as usize - 1][pos.x as usize]
    });

    if fallable {
        block_query.iter_mut().for_each(|(_, mut pos, _)| {
            pos.y -= 1;
        })
    } else {
        block_query.iter().for_each(|(entity, pos, _)| {
            commands.entity(entity).remove::<Free>();
            commands.entity(entity).insert(Fix);
            game_board.0[pos.y as usize][pos.x as usize] = true;
        });
        new_block_events_writer.send(NewBlockEvent);
    }
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
        .insert(position)
        .insert(Free);
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
                x: UNIT_WIDTH,
                y: UNIT_HEIGHT as f32,
                z: 0.,
            };
        });
}

fn block_horizontal_move(
    input: Res<Input<KeyCode>>,
    timer: ResMut<InputTimer>,
    game_board: ResMut<GameBoard>,
    mut block_query: Query<(Entity, &mut Position, &Free)>,
) {
    if !timer.0.finished() {
        return;
    }

    if input.pressed(KeyCode::Left) {
        let movable_left = block_query.iter_mut().all(|(_, pos, _)| {
            if pos.x == 0 {
                return false;
            }

            if pos.y as u32 >= Y_LENGTH {
                return pos.x > 0;
            }

            !game_board.0[pos.y as usize][pos.x as usize - 1]
        });

        if !movable_left {
            return;
        }

        block_query.iter_mut().for_each(|(_, mut pos, _)| {
            pos.x -= 1;
        })
    }

    if input.pressed(KeyCode::Right) {
        let movable_right = block_query.iter_mut().all(|(_, pos, _)| {
            if pos.x as u32 == X_LENGTH - 1 {
                return false;
            }

            if pos.y as u32 >= Y_LENGTH {
                return (pos.x as u32) < X_LENGTH - 1;
            }

            !game_board.0[pos.y as usize][pos.x as usize + 1]
        });

        if !movable_right {
            return;
        }

        block_query.iter_mut().for_each(|(_, mut pos, _)| {
            pos.x += 1;
        })
    }
}

fn block_vertical_move(
    input: Res<Input<KeyCode>>,
    mut game_board: ResMut<GameBoard>,
    mut block_query: Query<(Entity, &mut Position, &Free)>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let mut down_height = 0;
}
