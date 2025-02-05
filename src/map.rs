use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;


pub fn draw_map(canvas: &mut Canvas<Window>)-> Result<(), Box<dyn std::error::Error>>{
    let height = 800;
    let width = 600;
    // Dessin sur le canvas
    canvas.set_draw_color(Color::RGB(167, 167, 167)); 
    canvas.fill_rect(Rect::new(0, 0, height, width))?; // fond

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.fill_rect(Rect::new((height/3).try_into().unwrap(), 0, (height/3).try_into().unwrap(), 600))?; // route vertical 

    canvas.set_draw_color(Color::RGB(0, 0, 0)); // Vert
    canvas.fill_rect(Rect::new(0, (width/3).try_into().unwrap(), 800, (width/3).try_into().unwrap()))?; // route horizontal

//route horizontal
    canvas.set_draw_color(Color::RGB(255, 255, 255)); // Bleu
    canvas.draw_rect(Rect::new(0, 201, 800, 33))?; // Ligne

   
    canvas.draw_rect(Rect::new(0, 234, 800, 33))?; // Ligne


    canvas.draw_rect(Rect::new(0, 267, 800, 33))?; // Ligne


    canvas.draw_rect(Rect::new(0, 300, 800, 33))?; // Ligne


    canvas.draw_rect(Rect::new(0, 333, 800, 33))?; // Ligne


    canvas.draw_rect(Rect::new(0, 366, 800, 33))?; // Ligne

//route vertical
    let largeur = 44;
    canvas.draw_rect(Rect::new(267, 0, largeur, height))?; // Ligne


    canvas.draw_rect(Rect::new((267 + largeur*2).try_into().unwrap(), 0, largeur, height))?; // Ligne


    canvas.draw_rect(Rect::new((267 + largeur*3).try_into().unwrap(), 0, largeur, height))?;


    canvas.draw_rect(Rect::new((267 + largeur*4).try_into().unwrap(), 0, largeur, height))?;

   
    canvas.draw_rect(Rect::new((267 + largeur*5).try_into().unwrap(), 0, largeur, height))?;


    //carre du milieux
    canvas.set_draw_color(Color::RGB(0, 0, 0)); // Vert
    canvas.fill_rect(Rect::new((height/3).try_into().unwrap(), (width/3).try_into().unwrap(), ((height/3)+1).try_into().unwrap(), ((width/3)).try_into().unwrap()))?; // fond


    //draw line
    canvas.set_draw_color(Color::RGB(255, 255, 255)); // Bleu
    canvas.draw_line(((height/3).try_into().unwrap(), (width/3).try_into().unwrap()), (((height/3)+133).try_into().unwrap(),((width/3)).try_into().unwrap()) )?;


    canvas.draw_line((((height/3)+133).try_into().unwrap(), ((width/3)*2).try_into().unwrap()), (((height/3)*2).try_into().unwrap(),((width/3)*2).try_into().unwrap()) )?;

    canvas.draw_line((((height/3)).try_into().unwrap(), ((width/3)*2).try_into().unwrap()), (((height/3)).try_into().unwrap(),((width/2)).try_into().unwrap()) )?;

    canvas.draw_line((((height/3)*2).try_into().unwrap(), ((width/3)).try_into().unwrap()), (((height/3)*2).try_into().unwrap(),((width/2)).try_into().unwrap()) )?;
    Ok(())

}