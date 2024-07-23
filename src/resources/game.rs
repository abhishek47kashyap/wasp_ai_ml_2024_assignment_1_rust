use crate::resources::entity;

pub struct Game
{
    num_entities: i32,
    num_timesteps: i32
}

impl Game
{
    pub fn new() -> Self
    {
        Game{num_entities: 100, num_timesteps: 10000}
    }

    pub fn run(&self)
    {
        println!("Starting game!");
        println!("Ending game!");
    }
}