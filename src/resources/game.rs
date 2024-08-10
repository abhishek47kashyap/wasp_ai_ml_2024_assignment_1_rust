use crate::resources::entity::{Entity, EntityPosition, generate_random_position};

pub struct Game
{
    num_entities: usize,
    num_timesteps: usize
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
        let mut population = self.create_population();
        println!("Ending game!");
    }

    fn create_population(&self) -> Vec<Entity>
    {
        let mut population:Vec<Entity> = Vec::with_capacity(self.num_entities);

        for id in 1..=self.num_entities
        {
            population.push(
                Entity::new(
                    id as i32,
                    0.3,
                    5.0,
                    generate_random_position(20.0, 20.0)
                )
            );
        }

        population
    }
}