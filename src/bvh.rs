use crate::shape::{BoundingBox, Intersection, Ray, Shape};
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BVHTree<'a> {
    pub bounding_box: BoundingBox,
    pub left: Option<Arc<BVHTree<'a>>>,
    pub right: Option<Arc<BVHTree<'a>>>,
    pub leaf: Option<&'a Shape<'a>>,
}

impl<'a> BVHTree<'a> {
    pub fn construct_rec(shapes: &'a [Shape<'a>], dimension: i32) -> Option<Self> {
        if shapes.len() == 0 {
            None
        } else if shapes.len() == 1 {
            let shape = shapes.get(0).unwrap();
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
                            min_vector: bb.min_vector.min(b.min_vector),
                            max_vector: bb.max_vector.max(b.max_vector),
                        }),
                    });
            // TODO: Implement better algorithm for actually creating the tree.
            //let sorted_shapes: Vec<Shape<'a>> = shapes.iter().collect();
            //sorted_shapes.sort_by(|a, b| {
            //if a.bounding_box().centre().dim(dimension)
            //> b.bounding_box().centre().dim(dimension)
            //{
            //Ordering::Greater
            //} else {
            //Ordering::Less
            //}
            //});
            let right = match Self::construct(&shapes[0..shapes.len() / 2]) {
                Some(s) => Some(Arc::new(s)),
                None => None,
            };
            let left = match Self::construct(&shapes[shapes.len() / 2..shapes.len()]) {
                Some(s) => Some(Arc::new(s)),
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

    pub fn construct(shapes: &'a [Shape<'a>]) -> Option<Self> {
        Self::construct_rec(shapes, 0)
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<(Intersection, &'a Shape<'a>)> {
        let mut aggregate = Vec::<(Intersection, &'a Shape<'a>)>::new();
        if self.bounding_box.intersects_with(ray) {
            if let Some(leaf_shape) = &self.leaf {
                if let Some(intersection) = leaf_shape.do_intersect(ray) {
                    aggregate.push((intersection, leaf_shape));
                }
            }
            if let Some(right) = &self.right {
                aggregate.extend(right.intersect(ray));
            }
            if let Some(left) = &self.left {
                aggregate.extend(left.intersect(ray));
            }
        }
        aggregate
    }
}
