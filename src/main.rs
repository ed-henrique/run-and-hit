use bevy::prelude::*;

const PLAYER_SPEED: f32 = 500.0;
const PLAYER_ACCEL: f32 = 30.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec2);

pub struct GamePlugin;

fn player_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_color(Color::srgb(0.9, 0.3, 0.7), Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, 10.0, 0.0),
            scale: Vec2::new(20.0, 20.0).extend(1.0),
            ..default()
        },
        Player,
        Velocity(Vec2::ZERO),
    ));
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_transform: Single<&mut Transform, With<Player>>,
    mut player_velocity: Single<&mut Velocity, With<Player>>,
    time: Res<Time>,
) {
    let mut dir = Vec2::ZERO;
    let vel = player_velocity.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        dir.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        dir.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        dir.y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        dir.y -= 1.0;
    }

    dir = dir.normalize_or_zero();
    dir = dir * PLAYER_SPEED;

    player_velocity.0 = vel.lerp(dir, PLAYER_ACCEL * time.delta_secs());

    // Update position with frame-rate independent movement
    player_transform.translation.x += vel.x * time.delta_secs();
    player_transform.translation.y += vel.y * time.delta_secs();
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup);
        app.add_systems(Update, move_player);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}
