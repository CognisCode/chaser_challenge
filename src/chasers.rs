use nannou::prelude::*;
use crate::*;
use crate::rewards::Reward;
use crate::close_strategy::chase_closest;
use crate::high_strategy::chase_highest;
use crate::value_strategy::chase_value;
use crate::custom_strategy::chase_custom;

#[derive(Debug, PartialEq, Clone)]
pub enum ChaserType {
    Closest,
    Highest,
    Value,
    Custom,
}

#[derive(Debug, Clone)]
pub struct Chaser {
    pub position: Vec2,
    pub angle_vec: Vec2,
    pub chaser_type: ChaserType,
    pub direction: Vec2,
    size: f32, 
    max_step_size: f32,
    pub target_id: i16,
    pub score: i32,
    pub color: Color,
}

impl Chaser {
    pub fn new(chaser_type: ChaserType, color: Color) -> Chaser {
     
        Chaser {
                position: vec2((random_f32() - 0.5) * WIDTH, (random_f32() - 0.5) * HEIGHT),
                angle_vec: vec2(0.0, 0.0),
                size: SIZE,
                direction: vec2(0.0, 0.0),
                max_step_size: 3.0,
                chaser_type,
                target_id: 0,
                score: 0,
                color
        }
    }

    pub fn show(&self, draw: &Draw) {
        draw.tri()
            .w_h(self.size, self.size)
            .x_y(self.position.x, self.position.y)
            .rotate(self.angle_vec.angle()) 
            .rgba(self.color.red, self.color.green, self.color.blue, 0.85);
    }

    pub fn update(&mut self){
        self.position += self.direction.normalize() * SPEED; 

        // angle vector is previous direction + new direction and then normalized to keep te step size equal to max step size
        self.angle_vec += self.direction;
        self.angle_vec = self.angle_vec.clamp_length(self.max_step_size,self.max_step_size);
        self.direction = vec2(0.0, 0.0);
    }

    pub fn strategy(&mut self, rewards: &Vec<Reward>, chasers: &Vec<Chaser>) {

        match self.chaser_type { 
            ChaserType::Closest => chase_closest(self, rewards),
            ChaserType::Value => chase_value(self, rewards),
            ChaserType::Highest  => chase_highest(self, rewards),
            ChaserType::Custom  => chase_custom(self, rewards, chasers)
        }
    }


}