use std::cmp::PartialOrd;
use std::ops::Add;

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: PartialOrd + Copy + Add<Output = T> + From<u8>,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let all_positive = sides.iter().all(|&side| side > T::from(0));
        let mut sorted_sides = sides;
        sorted_sides.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let triangle_inequality = (sorted_sides[0] + sorted_sides[1]) > sorted_sides[2];
        if all_positive && triangle_inequality {
            Some(Triangle {
                sides: sorted_sides,
            })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1] && self.sides[1] != self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1]
            || self.sides[1] == self.sides[2]
            || self.sides[0] == self.sides[2]
    }
}
