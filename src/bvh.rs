use std::sync::Arc;
use std::cmp::Ordering;

use rand::Rng;

use crate::{
    ray::{Ray, Intersect},
    interval::Interval,
    hittable::{Hittable, HitRecord, Primitive},
    hittable_list::HittableList,
    aabb::AABB,
};

pub enum BvhNode {
    Leaf(Primitive),
    Node {
        left:  Arc<BvhNode>,
        right: Arc<BvhNode>,
        bbox: AABB,
    }
}

impl BvhNode {
    pub fn new(list: &mut HittableList<Primitive>) -> Self {
        Self::from_vec(&mut list.objects)
    }

    pub fn from_vec(list: &mut [Primitive]) -> Self {
        use BvhNode::*;
        let start = 0;
        let end = list.len();

        let axis = rand::thread_rng().gen_range(0..=2);
        let comparator = match axis { 
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ =>  Self::box_z_compare 
        };

        let object_span = end - start;

        let (left, right): (BvhNode, BvhNode) = match object_span {
            1 => (Leaf(list[start].clone()), Leaf(list[start].clone())),
            2 => if comparator(&list[start], &list[start+1]).is_lt() {
                    let left = list[start].clone();
                    let right = list[start+1].clone();
                    (Leaf(left), Leaf(right))
                } else {
                    let left = list[start+1].clone();
                    let right = list[start].clone();
                    (Leaf(left), Leaf(right))
                },
            _ => {
                list.sort_by(comparator);

                let mid = start + object_span/2;
                let left = Self::from_vec(&mut list[start..mid]);
                let right = Self::from_vec(&mut list[mid..end]);
                (left, right)
            }
        };
        let bbox = AABB::from_aabbs(&left.bounding_box(), &right.bounding_box());
        Node {
            left: Arc::new(left),
            right: Arc::new(right),
            bbox
        }
    }

    fn box_compare(a: &Primitive, b: &Primitive, axis_index: usize) -> Ordering {
        if a.bounding_box().axis(axis_index).min < b.bounding_box().axis(axis_index).min {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    fn box_x_compare(a: &Primitive, b: &Primitive) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Primitive, b: &Primitive) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Primitive, b: &Primitive) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let bbox = self.bounding_box();

        if !r.intersects(&bbox, ray_t) { return None }

        match self {
            BvhNode::Node { left, right, .. } => {
                let hit_left = left.hit(r, ray_t);
                let new_max = if let Some(hl) = &hit_left { hl.t } else {ray_t.max};
                let hit_right = right.hit(r, Interval::new(ray_t.min, new_max));

                match hit_right {
                    Some(_) => hit_right,
                    None => hit_left,
                }
            },
            BvhNode::Leaf(l) => l.hit(r, ray_t)
        }
    }

    fn bounding_box(&self) -> AABB {
        match self {
            Self::Leaf(l) => l.bounding_box(),
            Self::Node{ bbox, ..} => *bbox,
        }
    }
}

