extern crate sdl2;
mod map;
mod vehicles;
mod stats;
mod pause;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};


use crate::map::*;
use crate::vehicles::*;
use crate::stats::Statistics;
use crate::pause::*;

use rand::Rng;

fn main() -> Result<(), String> {
    let mut last_input_time = Instant::now(); 
    let cooldown_duration = Duration::from_millis(500); 
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("SDL2 Canvas en Rust", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;

    // Background colors
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let speed = vec![3, 4, 5];
    let left = vec![(0, 308, "Up".to_string()), (0, 340, "Right".to_string()), (0, 370, "Down".to_string())];
    let right = vec![(750, 274, "Down".to_string()), (750, 240, "Left".to_string()), (750, 210, "Up".to_string())];
    let up = vec![(275, 0, "Left".to_string()), (325, 0, "Down".to_string()), (365, 0, "Right".to_string())];
    let down = vec![(410, 580, "Left".to_string()), (450, 580, "Up".to_string()), (500, 580, "Right".to_string())];
    let mut vehicles = vec![];

    //Creation of the stats struct.
    let mut stats = Statistics::new(0,0,0,0.0,0.0);

    let mut paused = false;
    let mut last_time = Instant::now();
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        let delta_time = last_time.elapsed().as_secs_f32(); // Elapsed time in seconds (f32)
        last_time = Instant::now();

        for event in event_pump.poll_iter() {
            let mut rng = rand::rng(); //Random number generator
            let random_number = rng.random_range(0..3);
            let random_speed = rng.random_range(0..3);

            match event {
                // Key event listener
                Event::KeyDown { keycode: Some(key), .. } => {
                    if last_input_time.elapsed() >= cooldown_duration {
                        match key {
                            //Spawn vehicles in the chosen direction.
                            Keycode::Left => {
                                if !paused {
                                    last_input_time = Instant::now();
                                    vehicles.push(Vehicle::new(right[random_number].0, right[random_number].1, 40, 20, speed[random_speed], "Right".to_string(), right[random_number].2.clone(), ));
                                }
                            }
                            Keycode::Right => {
                                if !paused {
                                    last_input_time = Instant::now();
                                    vehicles.push(Vehicle::new(left[random_number].0, left[random_number].1, 40, 20, speed[random_speed], "Left".to_string(), left[random_number].2.clone()));
                                }
                            }
                            Keycode::Up => {
                                if !paused {
                                    last_input_time = Instant::now();
                                    vehicles.push(Vehicle::new(down[random_number].0, down[random_number].1, 20, 40, speed[random_speed], "Down".to_string(), down[random_number].2.clone()));
                                }
                            }
                            Keycode::Down => {
                                if !paused {
                                    last_input_time = Instant::now();
                                    vehicles.push(Vehicle::new(up[random_number].0, up[random_number].1, 20, 40, speed[random_speed], "Up".to_string(), up[random_number].2.clone()));
                                }
                            }
                            Keycode::R => {
                                // Randomly spawn vehicles from all directions
                                if !paused {
                                    for _ in 0..5 {
                                        let direction = rand::random::<u8>() % 4; // Randomly choose a direction (0: left, 1: right, 2: up, 3: down)
                                        if last_input_time.elapsed() >= cooldown_duration {
                                            match direction {
                                                0 => {
                                                    last_input_time = Instant::now();
                                                    vehicles.push(Vehicle::new(right[random_number].0, right[random_number].1, 40, 20, speed[random_speed], "Right".to_string(), right[random_number].2.clone()));
                                                }
                                                1 => {
                                                    last_input_time = Instant::now();
                                                    vehicles.push(Vehicle::new(left[random_number].0, left[random_number].1, 40, 20, speed[random_speed], "Left".to_string(), left[random_number].2.clone()));
                                                }
                                                2 => {
                                                    last_input_time = Instant::now();
                                                    vehicles.push(Vehicle::new(down[random_number].0, down[random_number].1, 20, 40, speed[random_speed], "Down".to_string(), down[random_number].2.clone()));
                                                }
                                                3 => {
                                                    last_input_time = Instant::now();
                                                    vehicles.push(Vehicle::new(up[random_number].0, up[random_number].1, 20, 40, speed[random_speed], "Up".to_string(), up[random_number].2.clone()));
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                            }
                               Keycode::W => {
                                break 'running; // Quitter la fenêtre lorsque W est pressée
                            }
                            Keycode::Escape => {
                                paused = !paused; // Basculer entre pause et reprise
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        // Dessine le véhicule
        let all_vehicles = vehicles.clone();
        let _ = draw_map(&mut canvas);

if paused {
    //Creation of a pause panel
    pause_panel(&mut canvas, stats.clone())

} else {
    //Update of the vehicle positions and creation 
    for vehicle in &mut vehicles {
        vehicle.update(&all_vehicles.clone(), delta_time);
        vehicle.draw(&mut canvas,);
    }
}

// Removal of vehicles off the image.
vehicles.retain(|vehicle| {
    let is_off_screen = vehicle.x <= 0 || vehicle.x + vehicle.width as i32 >= 850 || vehicle.y <= 0 || vehicle.y + vehicle.height as i32 >= 650;
    
    if is_off_screen {
        stats.update_vehicule(); 
        stats.parametre(vehicle);
    }

    !is_off_screen
});
        // Update of the display
        canvas.present();

        // limitation to ~60 FPS
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
