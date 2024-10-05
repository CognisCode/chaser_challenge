use crate::rewards::*;
use crate::chasers::*;

use std::ops::Sub;

pub fn chase_custom(chaser: &mut Chaser ,rewards: &Vec<Reward>, other_chasers: &Vec<Chaser>){
    // chaser: struct object of the custom chaser
    // rewards: vector containing the reward objects (location and value of every reward can be accessed here)
    // other: vector containing the location (location and target id of every chaser can be accessed here)
    // to add memory the Chaser struct can be expanded
    
    // this one is really a bad approach
    // value = reward_value / distance_to_reward  * 1 / (1 + other_chaser_distance_to_reward)
    if rewards.len() == 0 { 
        return; 
     }
 
    let mut most_valuable = 0.0;
 
    for reward in rewards{
        let distance = reward.position.distance(chaser.position);
        
        for other in other_chasers {
            let other_distance = reward.position.distance(other.position);

            if reward.value as f32 / distance * 1.0 / (1.0 + other_distance) > most_valuable{
                chaser.direction = reward.position.sub(chaser.position).normalize();
                most_valuable = reward.value as f32 / distance * 1.0 / (1.0 + other_distance);
                chaser.target_id = reward.id; 
            }
        }
    }
}