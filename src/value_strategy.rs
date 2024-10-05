
use crate::rewards::*;
use crate::chasers::*;

use std::ops::Sub;
pub fn chase_value(chaser: &mut Chaser, rewards: &Vec<Reward>){
    // chase target with highest value/distance ratio
    
    if rewards.len() == 0 { 
       return; 
    }

    let mut most_valuable = 0.0;

    for reward in rewards {

        if reward.reward_type != RewardType::Consumed {
            
            if reward.value as f32 / reward.position.distance(chaser.position) > most_valuable {    
                chaser.direction = reward.position.sub(chaser.position).normalize();
                most_valuable = reward.value as f32 / reward.position.distance(chaser.position) ;
                chaser.target_id = reward.id; 
            }   
        }
    }
}


