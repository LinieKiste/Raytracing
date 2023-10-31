use std::{
    cmp::Ordering,
    sync::Arc,
};

use rand::Rng;

use crate::{
    ray::Ray,
    interval::Interval,
    hittable::{Hittable, HitRecord, Primitive},
    hittable_list::HittableList,
    aabb::AABB,
};

type ThreadSafeHittable = Arc<dyn Hittable + Sync + Send>;
pub struct BvhNode {
    left:  ThreadSafeHittable,
    right: ThreadSafeHittable,
    bbox: AABB,
}

impl BvhNode {
    pub fn new(list: &mut HittableList<Primitive>) -> Self {
        Self::from_vec(&mut list.objects)
    }

    pub fn from_vec(list: &mut [Primitive]) -> Self {
        let start = 0;
        let end = list.len();

        let axis = rand::thread_rng().gen_range(0..=2);
        let comparator = match axis { 
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ =>  Self::box_z_compare 
        };

        let object_span = end - start;

        let (left, right): (ThreadSafeHittable, ThreadSafeHittable) = match object_span {
            1 => (Arc::new(list[start]), Arc::new(list[start])),
            2 => if comparator(&list[start], &list[start+1]).is_lt() {
                    let left = list[start];
                    let right = list[start+1];
                    (Arc::new(left), Arc::new(right))
                } else {
                    let left = list[start+1];
                    let right = list[start];
                    (Arc::new(left), Arc::new(right))
                },
            _ => {
                list.sort_by(comparator);

                let mid = start + object_span/2;
                let left = Self::from_vec(&mut list[start..mid]);
                let right = Self::from_vec(&mut list[mid..end]);
                (Arc::new(left), Arc::new(right))
            }
        };
        let bbox = AABB::from_aabbs(&left.bounding_box(), &right.bounding_box());
        BvhNode { left, right, bbox }
    }

    fn box_compare(a: &Primitive, b: &Primitive, axis_index: u8) -> Ordering {
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
        if !self.bbox.hit(r, ray_t) { return None }

        let hit_left = self.left.hit(r, ray_t);
        let new_max = if let Some(hl) = &hit_left { hl.t } else {ray_t.max};
        let hit_right = self.right.hit(r, Interval::new(ray_t.min, new_max));

        match hit_right {
            Some(_) => hit_right,
            None => hit_left,
        }
    }

    fn bounding_box(&self) -> crate::aabb::AABB {
        self.bbox
    }
}

