use nannou::prelude::*;
use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Reward {
    pub position: Vec2,
    pub reward_type: RewardType,
    size: f32, 
    pub id: i16,
    pub value: i32,
    pub color: Color,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RewardType {
    Consumed,
    HighReward,
    LowReward
}

impl Reward {

    pub fn new(reward_type: RewardType, id: i16, value: i32, color: Color) -> Reward {
        Reward {
                position: vec2((random_f32() - 0.5) * WIDTH, (random_f32() - 0.5) * HEIGHT),
                size: SIZE,
                reward_type,
                id,
                value,
                color
        }
    }

    pub fn show(&self, draw: &Draw) {
        draw.ellipse()
            .w_h(self.size, self.size)
            .x_y(self.position.x, self.position.y)
            .rgba(self.color.red, self.color.green, self.color.blue, 0.85);
    }

    pub fn assign_score(&mut self, chasers: &mut Vec<Chaser>) -> bool {

        if chasers.len() == 0 {
            return false;
        }
        
        let mut consumed: bool = false;

        for chaser in chasers {

            if self.position.distance(chaser.position) < 5.0 && self.id == chaser.target_id {
                chaser.score += self.value;
                chaser.target_id = 0;
                consumed = true;
            }
        }

        if consumed {
            self.reward_type = RewardType::Consumed;
            self.color = Color{red: 255.0, green: 255.0, blue: 0.0};
            self.value = 0;
            return true
        }
        return false;
    }
}