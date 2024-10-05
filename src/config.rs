pub const VALUECHASER: usize = 1;
pub const CLOSECHASER: usize = 1;
pub const HIGHCHASER: usize = 1;
pub const CUSTOMCHASER: usize = 1;
pub const HIGHREWARDS: usize = 100;
pub const LOWREWARDS: usize = 100;
pub const MAXITERATIONS: usize = 30; // simulation will run for 30 rounds before exiting
pub const HIGHVALUE: i32 = 200;
pub const LOWVALUE: i32 = 50;
pub const PORT: i32 = 9007;
pub const WIDTH: f32 = 2400.0; // adjust for your screen size
pub const HEIGHT: f32 = 1200.0; // adjust for your screen size
pub const SIZE: f32 = 15.0;
pub const SPEED: f32 = 10.0;
pub const SENDOSCDATA: bool = false;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
