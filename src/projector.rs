use glam::DVec2;

pub trait Projector {
    fn project(&self, v: DVec2) -> DVec2;
    fn project_scalev(&self, v: DVec2) -> DVec2;
    fn project_scale(&self, n: f64) -> f64;

    fn unproject(&self, v: DVec2) -> DVec2;
    fn unproject_scalev(&self, v: DVec2) -> DVec2;
    fn unproject_scale(&self, n: f64) -> f64;
}
