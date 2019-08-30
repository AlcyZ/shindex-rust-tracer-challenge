use crate::ray::Ray;
use crate::tuple::{dot, point, tuple_subtract};
use crate::sphere::Sphere;

pub fn intersect(sphere: &Sphere, ray: &Ray) -> Option<[f64; 2]> {
    let sphere_to_ray = tuple_subtract(ray.origin, point(0.0, 0.0, 0.0));

    let a = dot(ray.direction, ray.direction);
    let b = 2.0 * dot(ray.direction, sphere_to_ray);
    let c = dot(sphere_to_ray, sphere_to_ray) - 1.0;

    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    }

    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

    Some([t1, t2])
}

#[cfg(test)]
mod tests {
    #[test]
    fn easy() {}
}