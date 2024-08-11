use rand::Rng;

#[derive(Clone)]
pub struct EntityPosition
{
    x: f64,
    y: f64
}

impl EntityPosition
{
    pub fn get_x(&self) -> f64
    {
        self.x
    }

    pub fn get_y(&self) -> f64
    {
        self.y
    }

    pub fn set_position(&mut self, pos: EntityPosition)
    {
        self.x = pos.x;
        self.y = pos.y;
    }
}

#[derive(Clone)]
pub struct Entity
{
    id: i32,
    size: f64,  // similar to radius in Python code
    current_position: EntityPosition,

    is_root: bool,
    has_converged: bool
}

impl Entity
{
    pub fn new(id: i32, size: f64, current_position: EntityPosition) -> Self
    {
        // https://doc.rust-lang.org/nomicon/constructors.html

        Entity
        {
            id,
            size,
            current_position,
            is_root: false,
            has_converged: false
        }
    }

    pub fn get_id(&self) -> i32
    {
        self.id
    }

    pub fn has_converged(&self) -> bool
    {
        self.has_converged
    }

    pub fn is_root(&self) -> bool
    {
        self.is_root
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