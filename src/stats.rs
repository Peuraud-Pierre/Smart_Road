use crate::Vehicle;

#[derive(Clone, Debug)]
pub struct Statistics {
   pub nbr_vehicule:  i32,
   pub max_velocity: i32,
   pub min_velocity: i32,
   pub max_time: f32,
   pub min_time: f32,

}
impl Statistics {
    pub fn new(nbr_vehicule: i32, max_velocity: i32, min_velocity: i32, max_time: f32, min_time: f32, ) -> Self {
        Self { nbr_vehicule, max_velocity, min_velocity, max_time, min_time }
    }

    pub fn update_vehicule(&mut self) {
        self.nbr_vehicule += 1;
    }
    // This function is for updating the stats of the party.
    pub fn parametre(&mut self,  vehicle: &Vehicle) {
            
                if self.min_velocity == 0 {
                    self.min_velocity = vehicle.speed;
                }else if self.min_velocity > vehicle.speed{
                    self.min_velocity = vehicle.speed;
                }

                if self.max_velocity < vehicle.speed{
                    self.max_velocity = vehicle.speed;
                }

                if self.max_time < vehicle.chronos{
                    self.max_time = vehicle.chronos;
                }

                if self.min_time == 0.0 {
                    self.min_time = vehicle.chronos;
                }else if self.min_time > vehicle.chronos{
                    self.min_time = vehicle.chronos;
                }
        }
    }