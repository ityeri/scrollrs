use crate::camera::Camera;
use crate::projector::Projector;
use glam::DVec2;

#[derive(Clone)]
pub struct ScrollAdaptor {
    pub camera: Camera,

    pub goal_zoom: f64,
    pub zoom_softness: f64,

    pub screen_center: DVec2,
    pub screen_radius: f64,
}

impl ScrollAdaptor {
    pub fn new(
        screen_width: f64,
        screen_height: f64,

        zoom_softness: Option<f64>,

        screen_center: Option<DVec2>,
        screen_radius: Option<f64>,
    ) -> Self {
        Self {
            camera: Camera {
                position: DVec2::new(0.0, 0.0),
                zoom: 1.0,
            },

            goal_zoom: 1.0,
            zoom_softness: zoom_softness.unwrap_or(0.05),

            screen_center: screen_center
                .unwrap_or(DVec2::new(screen_width / 2.0, screen_height / 2.0)),
            screen_radius: screen_radius.unwrap_or(if screen_height < screen_width {
                screen_width / 2.0
            } else {
                screen_height / 2.0
            }),
        }
    }
    pub fn resized(
        &self,
        screen_width: f64,
        screen_height: f64,

        screen_center: Option<DVec2>,
        screen_radius: Option<f64>,
    ) -> Self {
        Self {
            screen_center: screen_center
                .unwrap_or(DVec2::new(screen_width / 2.0, screen_height / 2.0)),
            screen_radius: screen_radius.unwrap_or(if screen_height < screen_width {
                screen_width / 2.0
            } else {
                screen_height / 2.0
            }),

            ..self.clone()
        }
    }

    pub fn moved(&mut self, v: DVec2) -> Self {
        self.camera.position += v;
        self.clone()
    }

    pub fn updated(&mut self, dt: f64) -> Self {
        let zoom_delta = self.goal_zoom - self.camera.zoom;
        let zoom_factor = 1.0 / self.zoom_softness * dt;

        if zoom_factor < 1.0 {
            self.camera.zoom += zoom_delta * zoom_factor
        } else {
            self.camera.zoom += zoom_delta;
        }

        self.clone()
    }

    pub fn zoom(&mut self, zoom: f64) -> Self {
        self.goal_zoom *= 1.0 + zoom;
        self.clone()
    }
    pub fn set_zoom(&mut self, zoom: f64) -> Self {
        self.goal_zoom = zoom;
        self.clone()
    }
}

impl Projector for ScrollAdaptor {
    fn project(&self, v: DVec2) -> DVec2 {
        let v = self.camera.project(v);
        DVec2::new(v.x, -v.y) * self.screen_radius + self.screen_center
    }
    fn project_scalev(&self, v: DVec2) -> DVec2 {
        let v = self.camera.project_scalev(v);
        DVec2::new(v.x, -v.y) * self.screen_radius
    }
    fn project_scale(&self, n: f64) -> f64 {
        self.camera.project_scale(n) * self.screen_radius
    }

    fn unproject(&self, v: DVec2) -> DVec2 {
        let v = (v - self.screen_center) / self.screen_radius;
        self.camera.unproject(DVec2::new(v.x, -v.y))
    }
    fn unproject_scalev(&self, v: DVec2) -> DVec2 {
        self.camera
            .unproject_scalev(DVec2::new(v.x, -v.y) / self.screen_radius)
    }
    fn unproject_scale(&self, n: f64) -> f64 {
        self.camera.unproject_scale(n / self.screen_radius)
    }
}
