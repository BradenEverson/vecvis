//! Vector Function Utility Structs

use std::{collections::VecDeque, ops::Range};

use bevy::prelude::Resource;

/// A set of points to draw the function with
#[derive(Resource, Default)]
pub struct PointCollection {
    /// The vector function defining the points
    points: VecDeque<(f32, f32, f32)>,
}

impl PointCollection {
    /// Fills the points of the collection with function values from a to b, stepping by `step`
    /// each time
    pub fn fill_span<FN: Fn(f32) -> (f32, f32, f32)>(
        &mut self,
        function: FN,
        range: Range<usize>,
        step: f32,
    ) {
        let a = range.start;
        let b = range.end;

        let steps = (((b - a) as f32) / step) as usize;

        for t in 0..steps {
            let t_here = a as f32 + (t as f32 * step);
            let coords = (function)(t_here);
            self.points.push_front(coords)
        }
    }
}

impl Iterator for PointCollection {
    type Item = (f32, f32, f32);
    fn next(&mut self) -> Option<Self::Item> {
        self.points.pop_back()
    }
}
