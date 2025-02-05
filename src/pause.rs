use sdl2::rect::Rect;
use sdl2::pixels::Color;

use crate::stats::Statistics;

pub fn pause_panel(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, stats: Statistics){
    let fixed_text_width = 200; 
    let fixed_text_height = 25; 
    let pause_width = 300;
    let pause_height = 200;
    let screen_width = 800;
    let screen_height = 600;

    let x = (screen_width - pause_width) / 2;
    let y = (screen_height - pause_height) / 2;

    canvas.set_draw_color(Color::RGBA(100, 100, 100, 180));
    let _ = canvas.fill_rect(Rect::new(x, y, pause_width.try_into().unwrap(), pause_height.try_into().unwrap()));

    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font("arial.ttf", 32).unwrap();
    let texture_creator = canvas.texture_creator();

    let surface = font.render("Pause")
        .blended(Color::RGB(255, 255, 255))
        .unwrap();
    
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

    let text_width = surface.width() - 40 ;
    let text_height = surface.height() -20;

    let text_x = x + (pause_width - text_width as i32) / 2;
    let text_y = (y + (pause_height - text_height as i32) / 2)-75;

    let _ = canvas.copy(&texture, None, Rect::new(text_x as i32, text_y as i32, text_width, text_height));


    let text = format!("Number of vehicles: {}", stats.nbr_vehicule);

    let surface_nbr_vehicules = font.render(&text)
        .blended(Color::RGB(255, 255, 255))
        .unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface_nbr_vehicules).unwrap();
    
    let text_x = (x + (pause_width - fixed_text_width as i32)/ 2) -50;
    let text_y = (y + (pause_height - fixed_text_height as i32) / 2) - 50;

    let _ = canvas.copy(&texture, None, Rect::new(text_x as i32, text_y as i32, fixed_text_width, fixed_text_height));



    let text_velocity = format!("Max velocity: {}", stats.max_velocity);

    let surface_vehicule_velocity= font.render(&text_velocity)
        .blended(Color::RGB(255, 255, 255))
        .unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface_vehicule_velocity).unwrap();

    
    let text_x = (x + (pause_width - fixed_text_width as i32)/ 2) -50;
    let text_y = (y + (pause_height - fixed_text_height as i32) / 2) - 30;

    let _ = canvas.copy(&texture, None, Rect::new(text_x as i32, text_y as i32, fixed_text_width - 25, fixed_text_height));


    let text_velocity = format!("Min velocity: {}", stats.min_velocity);

    let surface_vehicule_minvelocity= font.render(&text_velocity)
        .blended(Color::RGB(255, 255, 255))
        .unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface_vehicule_minvelocity).unwrap();

    
    let text_x = (x + (pause_width - fixed_text_width as i32)/ 2) -50;
    let text_y = (y + (pause_height - fixed_text_height as i32) / 2) - 10;

    let _ = canvas.copy(&texture, None, Rect::new(text_x as i32, text_y as i32, fixed_text_width - 25, fixed_text_height ));


    let text = format!("Max time: {}", stats.max_time);

    let surface_nbr_vehicules = font.render(&text)
        .blended(Color::RGB(255, 255, 255))
        .unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface_nbr_vehicules).unwrap();
    
    let text_x = (x + (pause_width - fixed_text_width as i32)/ 2) -50;
    let text_y = (y + (pause_height - fixed_text_height as i32) / 2) + 10;

    let _ = canvas.copy(&texture, None, Rect::new(text_x as i32, text_y as i32, fixed_text_width - 25, fixed_text_height));

    let text = format!("Min time: {}", stats.min_time);

    let surface_nbr_vehicules = font.render(&text)
        .blended(Color::RGB(255, 255, 255))
        .unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface_nbr_vehicules).unwrap();
    
    let text_x = (x + (pause_width - fixed_text_width as i32)/ 2) -50;
    let text_y = (y + (pause_height - fixed_text_height as i32) / 2) + 30;

    let _ = canvas.copy(&texture, None, Rect::new(text_x as i32, text_y as i32, fixed_text_width - 25, fixed_text_height));

    let text = format!("Close calls: 0");

    let surface_nbr_vehicules = font.render(&text)
        .blended(Color::RGB(255, 255, 255))
        .unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface_nbr_vehicules).unwrap();
    
    let text_x = (x + (pause_width - fixed_text_width as i32)/ 2) -50;
    let text_y = (y + (pause_height - fixed_text_height as i32) / 2) + 50;

    let _ = canvas.copy(&texture, None, Rect::new(text_x as i32, text_y as i32, fixed_text_width - 25, fixed_text_height));

    let text = format!("Min time: {}", stats.min_time);

    let surface_nbr_vehicules = font.render(&text)
        .blended(Color::RGB(255, 255, 255))
        .unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface_nbr_vehicules).unwrap();
    
    let text_x = (x + (pause_width - fixed_text_width as i32)/ 2) -50;
    let text_y = (y + (pause_height - fixed_text_height as i32) / 2) + 30;

    let _ = canvas.copy(&texture, None, Rect::new(text_x as i32, text_y as i32, fixed_text_width - 25, fixed_text_height));

}