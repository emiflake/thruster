use crate::shape::{BoundingBox, Intersection, Ray, SceneObject, Shape};
use std::cmp::Ordering;

/// A Tree that uses a [Shapes](../shape/enum.Shape.html) [BoundingBox](../shape/struct.BoundingBox.html)
/// to index and find the intersection.
pub struct BVHTree {
    pub bounding_box: BoundingBox,
    pub left: Option<Box<BVHTree>>,
    pub right: Option<Box<BVHTree>>,
    pub leaf: Option<Shape>,
}

impl BVHTree {
    fn construct_rec(mut shapes: Vec<Shape>, dimension: u32) -> Option<Self> {
        if shapes.len() == 0 {
            None
        } else if shapes.len() == 1 {
            let shape = shapes.get(0).unwrap().clone();
            Some(Self {
                bounding_box: shape.bounding_box(),
                left: None,
                right: None,
                leaf: Some(shape),
            })
        } else {
            let bb =
                shapes
                    .iter()
                    .map(|shape| shape.bounding_box())
                    .fold(None, |acc, b| match acc {
                        None => Some(b),
                        Some(bb) => Some(BoundingBox {
                            min_vector: bb.min_vector.min(&b.min_vector),
                            max_vector: bb.max_vector.max(&b.max_vector),
                        }),
                    });
            shapes.sort_by(|a, b| {
                if a.bounding_box().centre().dim(dimension)
                    > b.bounding_box().centre().dim(dimension)
                {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });
            // TODO: Implement better algorithm for actually creating the tree.
            let right =
                match Self::construct_rec(shapes[0..shapes.len() / 2].to_vec(), dimension + 1) {
                    Some(s) => Some(Box::new(s)),
                    None => None,
                };
            let left = match Self::construct_rec(
                shapes[shapes.len() / 2..shapes.len()].to_vec(),
                dimension + 1,
            ) {
                Some(s) => Some(Box::new(s)),
                None => None,
            };

            Some(Self {
                bounding_box: bb.unwrap(),
                right,
                left,
                leaf: None,
            })
        }
    }

    /// Construct a BVH tree from a slice of [Shapes](../shape/enum.Shape.html)
    pub fn construct(shapes: &[Shape]) -> Option<Self> {
        Self::construct_rec(shapes.to_vec(), 0)
    }

    /// Find intersection with [Ray](../shape/struct.Ray.html) in the tree
    /// Used for finding Ray-X intersections in a scene.
    pub fn intersect(&self, ray: &Ray) -> Option<(Intersection, &Shape)> {
        let mut closest = None;

        self.intersect_rec(ray, &mut closest);

        closest
    }

    fn intersect_rec<'a>(&'a self, ray: &Ray, closest: &mut Option<(Intersection, &'a Shape)>) {
        if self.bounding_box.intersects_with(ray) {
            if let Some(leaf_shape) = &self.leaf {
                if let Some(intersection) = leaf_shape.do_intersect(ray) {
                    match closest {
                        Some(i) => {
                            if intersection.t < i.0.t {
                                *closest = Some((intersection, leaf_shape));
                            }
                        }
                        _ => {
                            *closest = Some((intersection, leaf_shape));
                        }
                    }
                }
            }
            if let Some(right) = &self.right {
                right.intersect_rec(ray, closest);
            }
            if let Some(left) = &self.left {
                left.intersect_rec(ray, closest);
            }
        }
    }
}
