
use std::ops::Sub;

use algebra_traits::{AdditiveGroup, Torsor};

use super::Arrow;

// vector is equivalence class of arrows
// two arrows are equivalent if they have same length and direction

pub struct AbstractVector<Point> {
    arrow:Arrow<Point>
}

impl<Point> AbstractVector<Point> {
    pub fn new(arrow:Arrow<Point>) -> Self {
        Self{arrow}
    }

}

impl<Point:Clone+PartialEq+Sub<Output=V>+Torsor, V:AdditiveGroup> AbstractVector<Point> {

    pub fn put(self, pt:Point) -> Arrow<Point> {
        let (foot, head)=self.arrow.clone().into_foot_head();
        let newfoot=pt.clone();
        let newhead=pt+(head-foot);
        Arrow::new(newfoot, newhead)
    }
}