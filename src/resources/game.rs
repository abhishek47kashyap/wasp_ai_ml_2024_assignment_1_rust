use crate::resources::entity::{Entity, EntityPosition, generate_random_position};
use crate::resources::utils_math::euclidean_dist;

use rand::seq::SliceRandom;

pub struct Game
{
    num_entities: usize,
    num_timesteps: usize,
    perception_radius: f64
}

impl Game
{
    pub fn new() -> Self
    {
        Game{num_entities: 100, num_timesteps: 10000, perception_radius: 2.5}
    }

    pub fn run(&self)
    {
        println!("========= STARTING GAME =========");
        let mut population = self.create_population();
        println!("Population created");
        let triplets = self.create_triplets(&population);
        self.mark_root_entities(&triplets, &mut population);
        println!("Triplets created");
        println!("========= GAME HAS ENDED =========");
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
                    generate_random_position(20.0, 20.0)
                )
            );
        }

        population
    }

    fn create_triplets(&self, population: &Vec<Entity>) -> Vec<Vec<i32>>
    {
        let mut triplets:Vec<Vec<i32>> = vec![];

        for (idx, entity) in population.iter().enumerate()
        {
            // get all entities within view of the current entity
            let slice_upto = &population[..idx];
            let slice_after = &population[idx+1..];
            let other_entities: Vec<&Entity> = slice_upto.iter().chain(slice_after.iter()).collect();

            // out of the visible entities, randomly select two to form a triplet
            if other_entities.len() >= 2
            {
                // https://stackoverflow.com/a/42272866/6010333
                let random_selections: Vec<&&Entity> = other_entities.choose_multiple(&mut rand::thread_rng(), 2).collect();

                let triplet = vec![entity.get_id(), random_selections[0].get_id(), random_selections[1].get_id()];
                triplets.push(triplet);
            }
        }

        triplets
    }

    fn get_entity_from_id(&self, entity_id: &i32, population: &Vec<Entity>) -> Option<Entity>
    {
        for entity in population
        {
            if entity.get_id() == entity_id.clone()
            {
                return Some(entity.clone());
            }
        }

        None
    }

    fn mark_root_entities(&self, triplets: &Vec<Vec<i32>>, population: &mut Vec<Entity>)
    {
        for tpl in triplets
        {
            let first_id = tpl[0];

            for elem in population.iter_mut()
            {
                if elem.get_id() == first_id
                {
                    elem.mark_as_root();
                }
            }
        }
    }
}