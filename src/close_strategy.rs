use crate::rewards::*;
use crate::chasers::*;

use std::ops::Sub;


pub fn chase_closest(chaser: &mut Chaser, rewards: &Vec<Reward>){
    // chase closest target

    if rewards.len() == 0 { 
       return; 
    }

    let mut shortest_distance = 100000000.0;

    for reward in rewards {

        if reward.reward_type != RewardType::Consumed {
            let distance = reward.position.distance(chaser.position);

            if reward.position.distance(chaser.position) < shortest_distance {    
                chaser.direction = reward.position.sub(chaser.position).normalize();
                shortest_distance = distance;
                chaser.target_id = reward.id; 
            }   
        }
    }
}