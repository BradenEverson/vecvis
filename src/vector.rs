//! Vector Function Utility Structs

use std::{collections::VecDeque, ops::Range};

use bevy::prelude::Resource;

/// A set of points to draw the function with
#[derive(Resource)]
pub struct PointCollection<FN: Fn(f32) -> (f32, f32, f32)> {
    /// The vector function defining the points
    function: FN,
    /// Points in the current range for the function
    points: VecDeque<(f32, f32, f32)>,
}

impl<FN: Fn(f32) -> (f32, f32, f32)> PointCollection<FN> {
    /// Creates a new point collection from a 3 space function
    pub fn from_fn(function: FN) -> Self {
        Self {
            function,
            points: VecDeque::new(),
        }
    }

    /// Fills the points of the collection with function values from a to b, stepping by `step`
    /// each time
    pub fn fill_span(&mut self, range: Range<usize>, step: f32) {
        let a = range.start;
        let b = range.end;

        let steps = (((b - a) as f32) / step) as usize;

        for t in 0..steps {
            let t_here = a as f32 + (t as f32 * step);
            let coords = (self.function)(t_here);
            self.points.push_front(coords)
        }
    }
}

impl<FN: Fn(f32) -> (f32, f32, f32)> Iterator for PointCollection<FN> {
    type Item = (f32, f32, f32);
    fn next(&mut self) -> Option<Self::Item> {
        self.points.pop_back()
    }
}
