use chaser_challenge::simulation::*;

// chasers can only consume rewards if they have targeted the reward.id with chaser.id
fn main() {
    nannou::app(app).update(next_step).exit(end).run();
}
