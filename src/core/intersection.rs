use crate::geometry::geometry_information::GeometryInformation;

pub struct Intersection {
    pub geom: GeometryInformation,
}

impl Intersection {
    pub fn nearest(self, other: Intersection) -> Self {
        if self.geom.t > other.geom.t {
            self
        } else {
            other
        }
    }
}
