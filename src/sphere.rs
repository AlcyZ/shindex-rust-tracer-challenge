use crate::ray::Ray;

#[derive(Debug)]
pub struct Sphere {
    id: usize
}

impl std::cmp::PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub fn sphere() -> Sphere {
    Sphere { id: 1 }
}

#[cfg(test)]
mod tests {}