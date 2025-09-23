use glam::DVec2;

use crate::projector::Projector;

pub struct Camera {
    pub position: DVec2,
    pub zoom: f64,
}

impl Projector for Camera {
    fn project(&self, v: DVec2) -> DVec2 {
        let relative_pos = v - self.position;
        relative_pos * self.zoom
    }
    fn project_scalev(&self, v: DVec2) -> DVec2 {
        v * self.zoom
    }
    fn project_scale(&self, n: f64) -> f64 {
        n * self.zoom
    }

    fn unproject(&self, v: DVec2) -> DVec2 {
        let resized_pos = v / self.zoom;
        resized_pos + self.position
    }
    fn unproject_scalev(&self, v: DVec2) -> DVec2 {
        v / self.zoom
    }
    fn unproject_scale(&self, n: f64) -> f64 {
        n / self.zoom
    }
}
