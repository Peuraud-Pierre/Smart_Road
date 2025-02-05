
#[derive(Clone, Debug)]
pub struct Vehicle {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub from: String,
    pub to: String,
    pub speed: i32, 
    pub initial_speed: i32,
    pub chronos: f32,
}

impl Vehicle {
    pub fn new(x: i32, y: i32, width: u32, height: u32, speed: i32, from: String, to: String) -> Self {
        Self { x, y, width, height, from, to, speed, initial_speed: speed,  chronos: 0.0}
    }

//This function is used to steer the vehicles in the right direction, as well as to avoid collisions.
    pub fn update(&mut self, all: &Vec<Vehicle>, delta_time: f32) {
        let (mut dx, mut dy) = (0, 0);

        self.chronos += delta_time;
        match self.from.as_str() {
            "Right" => {
                if self.to == "Down" && self.x <= 370 {
                    self.width = 20;
                    self.height = 40;
                    dy = self.speed;
                } else if self.to == "Up" && self.x <= 500 {
                    self.width = 20;
                    self.height = 40;
                    dy = -self.speed;
                } else {
                    dx = -self.speed;
                }
            }
            "Left" => {
                if self.to == "Down" && self.x >= 280 {
                    self.width = 20;
                    self.height = 40;
                    dy = self.speed;
                } else if self.to == "Up" && self.x >= 410 {
                    self.width = 20;
                    self.height = 40;
                    dy = -self.speed;
                } else {
                    dx = self.speed;
                }
            }
            "Down" => {
                if self.to == "Right" && self.y <= 375 {
                    self.width = 40;
                    self.height = 20;
                    dx = self.speed;
                } else if self.to == "Left" && self.y <= 275 {
                    self.width = 40;
                    self.height = 20;
                    dx = -self.speed;
                } else {
                    dy = -self.speed;
                }
            }
            "Up" => {
                if self.to == "Right" && self.y >= 305 {
                    self.width = 40;
                    self.height = 20;
                    dx = self.speed;
                } else if self.to == "Left" && self.y >= 205 {
                    self.width = 40;
                    self.height = 20;
                    dx = -self.speed;
                } else {
                    dy = self.speed;
                }
            }
            _ => {}
        }

        // Detection of nearby vehicles in the direction of movement.
        let filtered_vehicles: Vec<&Vehicle> = all.iter()
            .filter(|v| {
                if dx != 0 {
                    v.y + v.height as i32 > self.y && v.y < self.y + self.height as i32 &&
                    ((dx > 0 && v.x > self.x) || (dx < 0 && v.x < self.x))
                } else {
                    v.x + v.width as i32 > self.x && v.x < self.x + self.width as i32 &&
                    ((dy > 0 && v.y > self.y) || (dy < 0 && v.y < self.y))
                }
            })
            .collect();
            //Locate the proxy vehicle
        let vehicle_in_front = if dx > 0 {
            filtered_vehicles.into_iter().min_by_key(|v| v.x)
        } else if dx < 0 {
            filtered_vehicles.into_iter().max_by_key(|v| v.x)
        } else if dy > 0 {
            filtered_vehicles.into_iter().min_by_key(|v| v.y)
        } else {
            filtered_vehicles.into_iter().max_by_key(|v| v.y)
        };

        // Avoid a collision 
        if let Some(vehicle) = vehicle_in_front {
            let next_x = self.x + dx;
            let next_y = self.y + dy;

            let collision = (next_x < vehicle.x + vehicle.width as i32 &&
                             next_x + self.width as i32 > vehicle.x) &&
                            (next_y < vehicle.y + vehicle.height as i32 &&
                             next_y + self.height as i32 > vehicle.y);

            if collision && self.to == vehicle_in_front.unwrap().to {
                //  self.speed == vehicle_in_front.unwrap().speed
                if self.initial_speed > 3{
                    self.initial_speed -= 1; 
                   }
            } else if collision{
                if (self.to != self.from) && (vehicle_in_front.unwrap().to == vehicle_in_front.unwrap().from) && collision{
                self.speed = 0 ;
                }
            }else {
                self.speed = self.initial_speed;
                self.x += dx;
                self.y += dy;
            }
        } else {
            self.x += dx;
            self.y += dy;
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0)); // Rouge
        let _ = canvas.fill_rect(sdl2::rect::Rect::new(self.x, self.y, self.width, self.height));
    }
}



//let vehicle = vehicle_in_front.unwrap();
                
                // Vérifier si le véhicule devant est bien avancé en fonction de la direction
                // let est_plus_avance = match self.to.as_str() {
                //     "Left" => vehicle.x < self.x,  // Si on va à gauche, il doit être à gauche
                //     "Right" => vehicle.x > self.x,  // Si on va à droite, il doit être à droite
                //     "Up" => vehicle.y < self.y,    // Si on va en haut, il doit être plus haut
                //     "Down" => vehicle.y > self.y,     // Si on va en bas, il doit être plus bas
                //     _ => false,
                // };