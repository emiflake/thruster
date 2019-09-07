// WARNING
//
// Please note the heavy 'inspiration' from the PBR book.
// Not all this code is completely the same, but it's very similar and should be used with care.
// It's the very backbone of the performance that Thruster has, and so it's the piece that needs
// most performance tweaking.

use crate::acceleration::queue_systems::FastStack;
use crate::algebra::prelude::*;
use crate::shape::{Intersection, Ray, SceneObject, Shape};
use crate::utils;

use std::cmp::Ordering;

/// Used to construct [BVHTree](struct.BVHTree.html)
pub struct BVHPrimitiveInfo {
    pub bounding_box: BoundingBox,
    pub centre: Vec3,
    pub index: usize,
}

/// Used to determine the best possible split using SAH
#[derive(Clone, Debug)]
pub struct BucketInfo {
    pub count: usize,
    pub bounding_box: BoundingBox,
}

impl Default for BucketInfo {
    fn default() -> Self {
        Self {
            count: 0,
            bounding_box: BoundingBox::EMPTY,
        }
    }
}

/// The approach to be used in constructing a BVH tree
#[derive(Clone, Copy)]
pub enum BVHConstructionAlgorithm {
    /// Basic and primitive way
    Middle,
    /// Split into Equal counts
    Equal,
    /// Surface Area Heuristic: One of the better systems for constructing optimal BVH trees. It
    /// constructs very optimal trees, but is slightly slower. Perfect for static scenes
    SAH,
}

impl BVHConstructionAlgorithm {
    pub fn perform_partitioning(
        self,
        primitive_info: &mut [BVHPrimitiveInfo],
        dimension: u32,
        aggregate_bounds: &BoundingBox,
        centroid_bounds: &BoundingBox,
    ) -> Option<usize> {
        match self {
            BVHConstructionAlgorithm::Middle => {
                let len = primitive_info.len();
                let pmid = (centroid_bounds.min_vector.dim(dimension)
                    + centroid_bounds.max_vector.dim(dimension))
                    / 2.0;
                let mut middle =
                    utils::partition(primitive_info, |pi| pi.centre.dim(dimension) < pmid);
                if middle == 0 || middle == len {
                    middle = len / 2;
                    pdqselect::select_by(primitive_info, middle, |pa, pb| {
                        if pa.centre.dim(dimension) > pb.centre.dim(dimension) {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        }
                    });
                }
                Some(middle)
            }
            BVHConstructionAlgorithm::Equal => {
                let middle = primitive_info.len() / 2;
                pdqselect::select_by(primitive_info, middle, |pa, pb| {
                    if pa.centre.dim(dimension) < pb.centre.dim(dimension) {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                });
                Some(middle)
            }
            // Surface-area Heuristic splitting method works
            // by finding the best possible place on an axis
            // to split the primitives.
            BVHConstructionAlgorithm::SAH => {
                let len = primitive_info.len();
                if len <= 4 {
                    let middle = len / 2;
                    pdqselect::select_by(primitive_info, middle, |pa, pb| {
                        if pa.centre.dim(dimension) > pb.centre.dim(dimension) {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        }
                    });
                    Some(middle)
                } else {
                    let bucket_amount = 12;
                    let mut buckets: Vec<BucketInfo> = vec![BucketInfo::default(); bucket_amount];
                    for info in primitive_info.iter() {
                        let b = (f64::from(bucket_amount as u32)
                            * centroid_bounds.offset(&info.centre).dim(dimension))
                            as usize;
                        let b = if b == bucket_amount {
                            bucket_amount - 1
                        } else {
                            b
                        };
                        assert!(b < bucket_amount);
                        buckets[b].count += 1;
                        buckets[b].bounding_box = buckets[b].bounding_box.merge(&info.bounding_box);
                    }
                    //println!(
                    //"Counts: {:?}",
                    //buckets
                    //.iter()
                    //.map(|bucket| bucket.count)
                    //.collect::<Vec<usize>>()
                    //);

                    let mut cost = vec![0.0; bucket_amount - 1];
                    for bucket_n in 0..bucket_amount - 1 {
                        let mut bounds_a = buckets[0].bounding_box.clone();
                        let mut bounds_b = buckets[bucket_n + 1].bounding_box.clone();
                        let mut count_a = 0;
                        let mut count_b = 0;
                        for bucket in buckets.iter().take(bucket_n - 1) {
                            bounds_a = bounds_a.merge(&bucket.bounding_box);
                            count_a += bucket.count;
                        }
                        for bucket in buckets.iter().skip(bucket_n - 1) {
                            bounds_b = bounds_b.merge(&bucket.bounding_box);
                            count_b += bucket.count;
                        }
                        //println!("Counts   : [{} A] - [{} B]", count_a, count_b);
                        //println!(
                        //"Bounds_SA: [{} A] - [{} B]",
                        //bounds_a.surface_area(),
                        //bounds_b.surface_area()
                        //);
                        cost[bucket_n] = 0.125
                            + (f64::from(count_a as u32) * bounds_a.surface_area()
                                + f64::from(count_b as u32) * bounds_b.surface_area())
                                / aggregate_bounds.surface_area();
                    }

                    let mut minimum = (cost[0], 0);
                    for (i, cost_elem) in cost.iter().enumerate().take(bucket_amount - 1).skip(1) {
                        if cost_elem < &minimum.0 {
                            minimum = (*cost_elem, i);
                        }
                    }
                    //println!("Chosen cost: {:?}", minimum);
                    //println!("Costs      : {:?}", cost);

                    let leaf_cost = len;
                    if leaf_cost > 16 || minimum.0 < f64::from(leaf_cost as u32) {
                        let middle = utils::partition(primitive_info, |pi: &BVHPrimitiveInfo| {
                            let b = (f64::from(bucket_amount as u32)
                                * centroid_bounds.offset(&pi.centre).dim(dimension))
                                as usize;
                            let b = if b >= bucket_amount {
                                bucket_amount - 1
                            } else {
                                b
                            };
                            b <= minimum.1
                        });
                        Some(middle)
                    } else {
                        None
                    }
                }
            }
        }
    }
}

impl Default for BVHConstructionAlgorithm {
    fn default() -> Self {
        Self::SAH
    }
}

/// A construction node for use in creating a BVHTree
#[derive(Debug)]
pub struct BVHBuildNode {
    /// Amount of primitives in this node
    pub primitive_amount: usize,
    /// Index of the first primivite
    pub primitive_index: Option<usize>,
    /// Axis we split on for this node
    pub split_axis: Option<u32>,
    /// Bounding box of the node
    pub bounding_box: BoundingBox,
    /// Left child
    pub left: Option<Box<BVHBuildNode>>,
    /// Right child
    pub right: Option<Box<BVHBuildNode>>,
}

impl BVHBuildNode {
    /// Constructor helper for leafs
    pub fn new_leaf(index: usize, primitive_amount: usize, bounds: BoundingBox) -> Self {
        Self {
            primitive_amount,
            primitive_index: Some(index),
            split_axis: None,
            bounding_box: bounds,
            left: None,
            right: None,
        }
    }

    /// Constructor helper for branches
    pub fn new_branch(axis: u32, left: Box<BVHBuildNode>, right: Box<BVHBuildNode>) -> Self {
        Self {
            primitive_amount: 0,
            primitive_index: None,
            split_axis: Some(axis),
            bounding_box: left.bounding_box.merge(&right.bounding_box),
            left: Some(left),
            right: Some(right),
        }
    }
}

/// A BVH-based Accelerator struct, used for constructing BVHTrees
pub struct BVHAccel {
    primitives: Vec<Shape>,
    algorithm: BVHConstructionAlgorithm,
}

impl BVHAccel {
    pub fn new(algorithm: BVHConstructionAlgorithm, primitives: Vec<Shape>) -> Self {
        Self {
            primitives,
            algorithm,
        }
    }

    pub fn construct(&mut self) -> Option<(usize, BVHBuildNode)> {
        if self.primitives.is_empty() {
            None
        } else {
            // Construct information required for building a BVHTree
            let mut primitive_info = Vec::new();
            for (i, shape) in self.primitives.iter().enumerate() {
                primitive_info.push(BVHPrimitiveInfo {
                    bounding_box: shape.bounding_box(),
                    centre: shape.bounding_box().centre(),
                    index: i,
                });
            }
            let mut total_nodes = 0;
            let mut ordered_shapes = Vec::new();

            let node =
                self.recursive_build(&mut primitive_info, &mut total_nodes, &mut ordered_shapes);
            self.primitives.swap_with_slice(&mut ordered_shapes);
            Some((total_nodes, node))
        }
    }

    /// Recursively build a [BVHBuildNode](struct.BVHBuildNode.html) by splitting with a
    /// particular algorithm
    pub fn recursive_build(
        &self,
        primitive_info: &mut [BVHPrimitiveInfo],
        total_nodes: &mut usize,
        ordered_shapes: &mut Vec<Shape>,
    ) -> BVHBuildNode {
        assert!(!primitive_info.is_empty());
        *total_nodes += 1;
        let aggregate_bounds: BoundingBox = primitive_info.iter().fold(
            BoundingBox::EMPTY,
            |acc: BoundingBox, b: &BVHPrimitiveInfo| acc.merge(&b.bounding_box),
        );

        let len = primitive_info.len();
        if len == 1 {
            let first_offset = ordered_shapes.len();
            for info in primitive_info {
                ordered_shapes.push(self.primitives[info.index].clone()); // TODO: Remove clone; this should be possible by using Rc during construction instead of pure Shapes
            }
            BVHBuildNode::new_leaf(first_offset, len, aggregate_bounds)
        } else {
            let centroid_bounds = primitive_info
                .iter()
                .fold(BoundingBox::EMPTY, |a: BoundingBox, b| {
                    a.merge_with_vec(&b.centre)
                });
            let dimension = centroid_bounds.max_extent();
            if (centroid_bounds.max_vector.dim(dimension)
                - centroid_bounds.min_vector.dim(dimension))
            .abs()
                < std::f64::EPSILON
            {
                // Centroid bounds are small, construct leaf node.
                let first_offset = ordered_shapes.len();
                for info in primitive_info {
                    ordered_shapes.push(self.primitives[info.index].clone()); // TODO: See above
                }
                BVHBuildNode::new_leaf(first_offset, len, aggregate_bounds)
            } else if let Some(middle) = self.algorithm.perform_partitioning(
                primitive_info,
                dimension,
                &aggregate_bounds,
                &centroid_bounds,
            ) {
                //println!("0 - {} - {}", middle, len);
                BVHBuildNode::new_branch(
                    dimension,
                    Box::new(self.recursive_build(
                        &mut primitive_info[..middle],
                        total_nodes,
                        ordered_shapes,
                    )),
                    Box::new(self.recursive_build(
                        &mut primitive_info[middle..],
                        total_nodes,
                        ordered_shapes,
                    )),
                )
            } else {
                let first_offset = ordered_shapes.len();
                for info in primitive_info {
                    ordered_shapes.push(self.primitives[info.index].clone()); // TODO: See above
                }
                BVHBuildNode::new_leaf(first_offset, len, aggregate_bounds)
            }
        }
    }

    pub fn flatten(self, node: Box<BVHBuildNode>, total_nodes: usize) -> BVHLinearTree {
        let mut tree = BVHLinearTree {
            linear_nodes: Vec::with_capacity(total_nodes),
            primitives: self.primitives,
        };
        let mut offset = 0;
        tree.flatten_from(node, &mut offset);
        tree
    }
}

#[derive(Debug)]
pub struct BVHLinearTree {
    pub linear_nodes: Vec<BVHLinearNode>,
    pub primitives: Vec<Shape>,
}

/// A compacted BVHNode for use in indexing
#[derive(Debug)]
pub struct BVHLinearNode {
    pub bounding_box: BoundingBox,
    pub primitive_amount: usize,
    pub node_content: usize,
    pub axis: u32,
}

impl Default for BVHLinearNode {
    fn default() -> Self {
        Self {
            bounding_box: BoundingBox::EMPTY,
            primitive_amount: 0,
            node_content: 0,
            axis: 0,
        }
    }
}

impl BVHLinearTree {
    pub fn flatten_from(&mut self, node: Box<BVHBuildNode>, offset: &mut usize) -> usize {
        self.linear_nodes.push(BVHLinearNode::default()); // FIXME: this should not be necessary
        let my_offset = *offset;
        *offset += 1;
        let linear_node = if node.primitive_amount > 0 {
            assert!(node.left.is_none() && node.right.is_none());
            assert!(node.primitive_amount < 2 << 16);
            // This is a leaf
            BVHLinearNode {
                node_content: node.primitive_index.unwrap(),
                primitive_amount: node.primitive_amount,
                bounding_box: node.bounding_box,
                axis: 0,
            }
        } else {
            assert_eq!(node.primitive_amount, 0);
            // This is a branch, so we will have to recurse
            self.flatten_from(node.left.unwrap(), offset);
            // We need idx to link to the distant node
            let idx = self.flatten_from(node.right.unwrap(), offset);
            BVHLinearNode {
                axis: node.split_axis.unwrap(),
                primitive_amount: 0,
                bounding_box: node.bounding_box,
                node_content: idx,
            }
        };
        *self.linear_nodes.get_mut(my_offset).unwrap() = linear_node;
        my_offset
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(Intersection, &Shape)> {
        let mut current_task = 0;
        let mut queue = FastStack::new();
        let mut closest: Option<(Intersection, &Shape)> = None;
        loop {
            let n = &self.linear_nodes[current_task];
            if n.bounding_box.intersects_with(ray) {
                if n.primitive_amount > 0 {
                    for i in 0..n.primitive_amount {
                        let prim = &self.primitives[i + n.node_content];
                        if let Some(intersection) = prim.do_intersect(ray) {
                            closest = match closest {
                                Some(i) => {
                                    if intersection.t < i.0.t {
                                        Some((intersection, prim))
                                    } else {
                                        Some(i)
                                    }
                                }
                                _ => Some((intersection, prim)),
                            }
                        }
                    }
                    match queue.pop() {
                        None => return closest,
                        Some(task) => current_task = task,
                    };
                } else if ray.inv_dir.dim(n.axis) < 0.0 {
                    queue.push(current_task + 1);
                    current_task = n.node_content;
                } else {
                    queue.push(n.node_content);
                    current_task += 1;
                }
            } else {
                match queue.pop() {
                    None => return closest,
                    Some(task) => current_task = task,
                };
            }
        }
    }
}
