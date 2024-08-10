use rand::Rng;

pub struct EntityPosition
{
    pub x: f64,
    pub y: f64
}

pub struct Entity
{
    id: i32,
    size: f64,  // similar to radius in Python code
    perception_radius: f64,
    current_position: EntityPosition,

    is_root: bool
}

impl Entity
{
    pub fn new(id: i32, size: f64, perception_radius: f64, current_position: EntityPosition) -> Self
    {
        // https://doc.rust-lang.org/nomicon/constructors.html

        Entity
        {
            id,
            size,
            perception_radius,
            current_position,
            is_root: false
        }
    }

    pub fn mark_as_root(&mut self)
    {
        self.is_root = true;
    }

    pub fn mark_as_not_root(&mut self)
    {
        self.is_root = false;
    }
}

impl EntityPosition
{
    pub fn new(x: f64, y: f64) -> Self
    {
        EntityPosition{x, y}
    }
}

pub fn generate_random_position(map_size_x: f64, map_size_y: f64) -> EntityPosition
{
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0.0..=map_size_x);
    let y = rng.gen_range(0.0..=map_size_y);
    EntityPosition { x, y }
}