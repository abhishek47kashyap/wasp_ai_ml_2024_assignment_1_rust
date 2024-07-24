use crate::resources::entity::{Entity, EntityPosition, generate_random_position};

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

    fn create_population(&self) -> Vec<Entity>
    {
        let mut population:Vec<Entity> = Vec::new();

        let position = generate_random_position(20.0, 20.0);

        population
    }
}