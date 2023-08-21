pub mod brdf;
mod evector;

use glam::Vec3;

pub trait ColorTypeFunctionality {
    fn max_to_one(&mut self);
}

impl ColorTypeFunctionality for Vec3 {
    fn max_to_one(&mut self) {
        let max = self.x.max(self.y).max(self.z);
        if max > 1.0 {
            *self /= max;
        }
    }
}
