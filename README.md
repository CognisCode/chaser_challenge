![](https://github.com/YassinYassinYassin/chaser_challenge/blob/main/resources/simulation.gif)


# Chaser Challenge
  
This program uses nannou to setup a visual simulation of a resource gathering game. Three chasers _value_, _close_, _high_ and _custom_ are generated. The goal is to accumelate as much rewards as possible. Their are high rewards labeled green worth 200 and low rewards labeled blue worth 50. 
- close: chases the closest resource regardless of value.
- high: chases high resources first. Only goes for low resources as all the high resources have been depleted.
- value: Chases the reward with the highest _value/distance_ ratio. 
- custom: Chases highest _reward_value / distance_to_reward  * 1 / (1 + other_chaser_distance_to_reward)_ ratio. Currently a really bad strategy. 

Can you change the strategy of custom into a winning strategy? Custom strategy needs to be altered in custom_strategy.rs

The program can send the scores and locations of each chaser through OSC data for other application by labeling SENDOSCDATA to true in the config.

