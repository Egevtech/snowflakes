use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use rand::*;

use raylib::math::Vector2;
use raylib::prelude::*;

const SPAWN_THREAD_ACTIVITY: u64 = 10; // Период между спавном снежинки
const GC_THREAD_ACTIVITY: u64 = 10; // Частота работы потока движения и очистки мусора
const MAX_FPS: u32 = 200; // Максимальное количество FPS

#[derive(Debug, Clone)]
struct Snowflake {
    position: Vector2,
    velocity: Vector2,
    radius: f32
}

impl Snowflake {
    fn random(screen_width: i16) -> Snowflake {
        let mut rng = rng();

        let start_pos_x: f32 = rng.random_range(0..screen_width) as f32;
        let start_pos_y: f32 = 0.0;

        let radius = rng.random_range(0.1..5.0);

        let vel_x = rng.random_range(-0.5..0.5) as f32;
        let vel_y = radius * rng.random_range(0.8..1.20);

        Snowflake::new(Vector2::new(start_pos_x, start_pos_y), Vector2::new(vel_x, vel_y), radius)

    }

    fn new(start: Vector2, vel: Vector2, rad: f32) -> Snowflake {
        Snowflake {
            position: start,
            velocity: vel,
            radius: rad
        }
    }
}

struct Snowflakes {
    pub snowflakes: Vec<Snowflake>,
    pub screen: Vector2
}

#[tokio::main]
async fn main() {
    let (width, height): (i16, i16) = (1024, 768);

    let (mut rl, thread) = raylib::init()
        .size(width as i32, height as i32)
        .title("Snowflakes")
        .transparent()
        .resizable()
        .build();

    let sf_buffer = Arc::new(Mutex::new(Snowflakes{snowflakes: vec![], screen: Vector2::new(width as f32, height as f32)}));

    let buffer_clone = Arc::clone(&sf_buffer);
    tokio::task::spawn_blocking(move || {
        loop {
            let mut guard = buffer_clone.lock().unwrap();
            let size = guard.screen.x;

            guard.snowflakes.push(Snowflake::random(size as i16));
            drop(guard);

            std::thread::sleep(tokio::time::Duration::from_millis(SPAWN_THREAD_ACTIVITY));
        }
    });

    let buffer_clone = Arc::clone(&sf_buffer);
    tokio::spawn(async move {

        let mut direct_time = std::time::SystemTime::now();

        loop {
            let mut guard = buffer_clone.lock().unwrap();

            if SystemTime::now().duration_since(direct_time).unwrap() > Duration::from_millis(GC_THREAD_ACTIVITY) {
                direct_time = SystemTime::now();

                guard.snowflakes.iter_mut().for_each(|s| {s.position.y += s.velocity.y; s.position.x += s.velocity.x;});

                let size = guard.screen.clone();
                let snow_count = guard.snowflakes.len();
                guard.snowflakes.retain(|s| s.position.x > 0.00 && s.position.x < size.x && s.position.y < size.y);
                let cleared = snow_count - guard.snowflakes.len();

                if cleared != 0 {
                    println!("{} snowflakes were removed", cleared);
                }
            }
        }
    });

    rl.set_target_fps(MAX_FPS);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(0, 0, 0, 100));

        let mut guard = sf_buffer.lock().unwrap();

        guard.screen = Vector2::new(d.get_screen_width() as f32, d.get_screen_height() as f32);

        for snowflake in guard.snowflakes.clone() {
            d.draw_circle(snowflake.position.x as i32, snowflake.position.y as i32, snowflake.radius, Color::WHITE);
        }

        d.draw_fps(0,0);
        d.draw_text(&format!("{} snowflakes", guard.snowflakes.len()), 00, 20, 20, Color::WHITE);

    }

    std::process::exit(0);
}
