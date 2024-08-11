use crate::resources::entity::EntityPosition;

pub fn euclidean_dist(a: EntityPosition, b: EntityPosition) -> f64
{
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).powf(0.5)
}