use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;

pub fn intersect(sphere: &Sphere, ray: &Ray) -> Option<[f64; 2]> {
    let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);

    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * ray.direction.dot(sphere_to_ray);
    let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

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