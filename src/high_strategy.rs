use crate::rewards::*;
use crate::chasers::*;
use crate::HIGHREWARDS;

use std::ops::Sub;


pub fn chase_highest(chaser: &mut Chaser, rewards: &Vec<Reward>){
    // chasest targets with high value first
    if rewards.len() == 0 { 
       return; 
    }

    let mut shortest_distance = 100000000.0;
    let mut highs_left: bool = false;

    for i in 0..rewards.len() {

        if rewards[i].reward_type == RewardType::HighReward && i < HIGHREWARDS {
            let distance = rewards[i].position.distance(chaser.position);

            if rewards[i].position.distance(chaser.position) < shortest_distance {    
                chaser.direction = rewards[i].position.sub(chaser.position).normalize();
                shortest_distance = distance;
                chaser.target_id = rewards[i].id; 
                highs_left = true;
            }   
        } else if !highs_left && rewards[i].reward_type == RewardType::LowReward {
            let distance = rewards[i].position.distance(chaser.position);

            if rewards[i].position.distance(chaser.position) < shortest_distance {    
                chaser.direction = rewards[i].position.sub(chaser.position).normalize();
                shortest_distance = distance;
                chaser.target_id = rewards[i].id;     
            }
        }
    }
}
