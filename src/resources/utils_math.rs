use crate::resources::entity::EntityPosition;

pub fn euclidean_dist(a: EntityPosition, b: EntityPosition) -> f64
{
    ((a.get_x() - b.get_x()).powi(2) + (a.get_y() - b.get_y()).powi(2)).powf(0.5)
}