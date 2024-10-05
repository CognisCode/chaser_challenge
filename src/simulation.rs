use nannou_osc as osc;
use nannou::prelude::*;

use crate::*;

pub struct Simulation {
    pub chasers: Vec<Chaser>,
    pub rewards: Vec<Reward> ,
    pub rewards_left: usize,
    pub iteration: usize,
    pub max_iterations: usize,
    pub sender: osc::Sender<osc::Connected>,
}

pub fn app(app: &App) -> Simulation {

    app.new_window()
        .size(2400, 1200)
        .view(view)
        .build()
        .unwrap();

    let iteration = 0;
    let max_iterations = MAXITERATIONS;
    let rewards_left = HIGHREWARDS + LOWREWARDS;

    let target_addr = format!("{}:{}", "127.0.0.1", PORT);
    let sender = osc::sender()
        .expect("Could not bind to default socket")
        .connect(target_addr)
        .expect("Could not connect to socket at address");


    let mut rewards: Vec<Reward> = Vec::with_capacity(HIGHREWARDS+LOWREWARDS);
    create_reward(1, HIGHREWARDS, RewardType::HighReward, HIGHVALUE, &mut rewards ,Color{red: 0.0, green: 255.0, blue: 0.0});
    create_reward(HIGHREWARDS as i16 + 1, LOWREWARDS, RewardType::LowReward, LOWVALUE, &mut rewards, Color{red: 0.0, green: 0.0, blue: 255.0});

    let mut chasers: Vec<Chaser> = Vec::with_capacity(CLOSECHASER + VALUECHASER + HIGHCHASER);
    create_chaser(CLOSECHASER, ChaserType::Closest, &mut chasers, Color{red: 139.0, green: 0.0, blue: 139.0});
    create_chaser(VALUECHASER, ChaserType::Value, &mut chasers,  Color{red: 0.0, green: 0.0, blue: 0.0});
    create_chaser(HIGHCHASER, ChaserType::Highest, &mut chasers,  Color{red: 255.0, green: 0.0, blue: 0.0});
    create_chaser(CUSTOMCHASER, ChaserType::Custom, &mut chasers, Color{red: 0.0, green: 0.0, blue: 139.0});

    Simulation {rewards_left, iteration, max_iterations, sender, chasers, rewards}
}

pub fn next_step(app: &App, simulation: &mut Simulation, _update: Update) {

    for i in 0..simulation.chasers.len() {

        let (before, rest) = simulation.chasers.split_at_mut(i);
        let (chaser, after) = rest.split_first_mut().unwrap();
        let other_chasers: Vec<Chaser> = before.iter().chain(after.iter()).cloned().collect();
        chaser.strategy(&simulation.rewards, &other_chasers);
    }

    for reward in  &mut simulation.rewards {
        if reward.assign_score(&mut simulation.chasers) {
            simulation.rewards_left -= 1;
        }
    }    

    for chaser in  &mut simulation.chasers {
        chaser.update();
    }

    if simulation.rewards_left == 0 {
            reset_all(simulation);
    }

    if simulation.iteration == simulation.max_iterations {
        app.quit();
        return; // critical for the app to gracefully close
    }
}

fn view(app: &App, simulation: &Simulation, frame: Frame) {

    let draw = app.draw();
    draw.background().color(BEIGE);

    for i in 0..simulation.chasers.len() {
        simulation.chasers[i].show(&draw); 
    }

    for i in 0..simulation.rewards.len() {
        simulation.rewards[i].show(&draw); 
    }

    // if you want to send osc data to another application 
    if SENDOSCDATA{
        send_data(&simulation.chasers, &simulation);
    }

    let close_score = format!("close score: {:.2}", simulation.chasers[0].score);
    let value_score = format!("value score: {:.2}", simulation.chasers[1].score);
    let high_score = format!("high score: {:.2}", simulation.chasers[2].score);
    let custom_score = format!("custom score: {:.2}", simulation.chasers[3].score);


    draw.text(&close_score)
        .x_y(-1100.0, 550.0)
        .color(MAGENTA)
        .font_size(24);

    draw.text(&value_score)
        .x_y(-1100.0, 500.0)
        .color(BLACK)
        .font_size(24);

    draw.text(&high_score)
        .x_y(-1100.0, 450.0)
        .color(RED)
        .font_size(24);

    draw.text(&custom_score)
        .x_y(-1100.0, 400.0)
        .color(BLUE)
        .font_size(24);

    draw.to_frame(app, &frame).unwrap();
}

pub fn reset_all(simulation: &mut Simulation) {

    simulation.iteration += 1;
    simulation.rewards_left =  HIGHREWARDS + LOWREWARDS;
    
    for chaser in &mut simulation.chasers {    
        chaser.score = 0;
        chaser.target_id = 0;
        chaser.position = vec2((random_f32() - 0.5) * WIDTH, (random_f32() - 0.5) * HEIGHT);
    }

    let mut high_reward_count = 0; 

    for reward in &mut simulation.rewards {
        
        if high_reward_count < HIGHREWARDS {
            reward.reward_type = RewardType::HighReward;
            reward.value = HIGHVALUE;
            reward.color = Color{red: 0.0, green: 255.0, blue: 0.0};
            reward.position = vec2((random_f32() - 0.5) * WIDTH, (random_f32() - 0.5) * HEIGHT);
            high_reward_count += 1;
        } else {
            reward.reward_type = RewardType::LowReward;
            reward.value = LOWVALUE;
            reward.color = Color{red: 0.0, green: 0.0, blue: 255.0};
            reward.position = vec2((random_f32() - 0.5) * WIDTH, (random_f32() - 0.5) * HEIGHT);
        }       
    }   
}

pub fn end(_app: &App, _model: Simulation) {
    println!("Exiting application...");
}

pub fn create_chaser(count: usize, chaser_type: ChaserType, all_chasers: &mut Vec<Chaser>, color: Color) {
    for _ in 0..count {
        let chaser = Chaser::new(chaser_type.clone(), color) ;
        all_chasers.push(chaser);
    }
}

pub fn create_reward(id_start: i16, count: usize, reward_type: RewardType, value: i32, all_rewards: &mut Vec<Reward>, color: Color) {
    let mut id = id_start;
    for _ in 0..count {
        let reward = Reward::new(reward_type.clone(), id, value, color);
        id += 1;
        all_rewards.push(reward);
    }
}

pub fn send_data(chasers: &Vec<Chaser>, simulation: &Simulation){
    
    let mut value_score: i32 = 0;
    let mut value_x: f32 = 0.0;
    let mut value_y: f32 = 0.0;

    let mut high_score: i32 = 0;
    let mut high_x: f32 = 0.0;
    let mut high_y: f32 = 0.0;

    let mut close_score: i32 = 0;
    let mut close_x: f32 = 0.0;
    let mut close_y: f32 = 0.0;

    let mut custom_score: i32 = 0;
    let mut custom_x: f32 = 0.0;
    let mut custom_y: f32 = 0.0;

    for chaser in chasers {
        match chaser.chaser_type {
            ChaserType::Closest => {
                close_score = chaser.score;
                close_x = chaser.direction[0];
                close_y = chaser.direction[1];
            },
            ChaserType::Highest => {
                high_score = chaser.score;
                high_x = chaser.direction[0];
                high_y = chaser.direction[1];
            },
            ChaserType::Value => {
                value_score = chaser.score;
                value_x = chaser.direction[0];
                value_y = chaser.direction[1];
            },
            ChaserType::Custom => {
                custom_score = chaser.score;
                custom_x = chaser.direction[0];
                custom_y = chaser.direction[1];
            },
        }
    }
    
    let args: Vec<nannou_osc::Type> = vec![
        osc::Type::Int(value_score),
        osc::Type::Int(high_score),
        osc::Type::Int(close_score),
        osc::Type::Int(custom_score),
        osc::Type::Float(value_x),
        osc::Type::Float(value_y),
        osc::Type::Float(high_x),
        osc::Type::Float(high_y),
        osc::Type::Float(close_x),
        osc::Type::Float(close_y),
        osc::Type::Float(custom_x),
        osc::Type::Float(custom_y)
        ]; 
        
    let osc_addr = "/simulation/scores".to_string();
    let packet = (osc_addr, args);
    simulation.sender.send(packet).ok();
}