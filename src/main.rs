mod cause_of_death;
mod error;
mod instance_counter;

pub use cause_of_death::{CauseOfDeath, CAUSES_OF_DEATH};
pub use error::{Error, Result};

fn main() {
    println!("Hello, world!");
}
