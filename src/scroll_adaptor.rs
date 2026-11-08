use crate::camera::Camera;
use crate::projector::Projector;
use glam::DVec2;

#[derive(Clone)]
pub struct ScrollAdapter {
    pub camera: Camera,

    pub goal_zoom: f64,
    pub zoom_softness: f64,

    /// Screen point (in screen coordinates) that stays fixed while the camera
    /// animates towards `goal_zoom`. `None` zooms around the screen center.
    pub zoom_anchor: Option<DVec2>,

    pub screen_center: DVec2,
    pub screen_radius: f64,
}

impl ScrollAdapter {
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

            zoom_anchor: None,

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

        let previous_zoom = self.camera.zoom;
        if zoom_factor < 1.0 {
            self.camera.zoom += zoom_delta * zoom_factor
        } else {
            self.camera.zoom += zoom_delta;
        }

        if let Some(anchor) = self.zoom_anchor {
            self.camera.position += self.anchor_offset(anchor, previous_zoom, self.camera.zoom);
        }

        self.clone()
    }

    pub fn zoom(&mut self, zoom: f64) -> Self {
        self.goal_zoom *= 1.0 + zoom;
        self.zoom_anchor = None;
        self.clone()
    }
    pub fn set_zoom(&mut self, zoom: f64) -> Self {
        self.goal_zoom = zoom;
        self.zoom_anchor = None;
        self.clone()
    }

    /// Same as `zoom`, but the world point currently under `screen_point`
    /// stays under it while the zoom animates.
    pub fn zoom_at(&mut self, zoom: f64, screen_point: DVec2) -> Self {
        self.goal_zoom *= 1.0 + zoom;
        self.zoom_anchor = Some(screen_point);
        self.clone()
    }
    /// Same as `set_zoom`, but the world point currently under `screen_point`
    /// stays under it while the zoom animates.
    pub fn set_zoom_at(&mut self, zoom: f64, screen_point: DVec2) -> Self {
        self.goal_zoom = zoom;
        self.zoom_anchor = Some(screen_point);
        self.clone()
    }

    /// World-space offset from the camera position to the point seen at
    /// `screen_point`, while the camera is at `zoom`.
    fn world_offset_at(&self, screen_point: DVec2, zoom: f64) -> DVec2 {
        let v = (screen_point - self.screen_center) / self.screen_radius;
        DVec2::new(v.x, -v.y) / zoom
    }

    /// How far the camera has to move so that the point seen at `screen_point`
    /// keeps its place on screen while the zoom goes from `from_zoom` to
    /// `to_zoom`.
    fn anchor_offset(&self, screen_point: DVec2, from_zoom: f64, to_zoom: f64) -> DVec2 {
        if !(to_zoom.is_finite() && to_zoom > 0.0) || from_zoom == to_zoom {
            return DVec2::ZERO;
        }

        self.world_offset_at(screen_point, from_zoom) * (1.0 - from_zoom / to_zoom)
    }
}

impl Projector for ScrollAdapter {
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

#[cfg(test)]
mod tests {
    use super::ScrollAdapter;
    use crate::projector::Projector;
    use glam::DVec2;

    const DT: f64 = 1.0 / 60.0;

    fn settled(mut adapter: ScrollAdapter, frames: usize) -> ScrollAdapter {
        for _ in 0..frames {
            adapter = adapter.updated(DT);
        }
        adapter
    }

    #[test]
    fn zoom_at_keeps_the_point_under_the_cursor_for_the_whole_animation() {
        let mut adapter = ScrollAdapter::new(500.0, 500.0, None, None, None);
        adapter = adapter.moved(DVec2::new(1.5, -0.75));

        let anchor = DVec2::new(400.0, 120.0);
        let before = adapter.unproject(anchor);

        adapter = adapter.zoom_at(0.5, anchor);

        for _ in 0..200 {
            adapter = adapter.updated(DT);
            let under_cursor = adapter.unproject(anchor);
            assert!(
                (under_cursor - before).length() < 1e-9,
                "point drifted while zooming: {under_cursor:?} vs {before:?}"
            );
        }

        assert!((adapter.camera.zoom - 1.5).abs() < 1e-9);
    }

    #[test]
    fn zoom_at_zooms_in_towards_the_cursor() {
        let adapter = ScrollAdapter::new(500.0, 500.0, None, None, None);
        let cursor = DVec2::new(500.0, 0.0);

        let zoomed = settled(adapter.clone().zoom_at(1.0, cursor), 200);

        // The point under the cursor is above and to the right of the camera
        // center, so zooming in towards it has to move the camera that way.
        assert!(zoomed.camera.position.x > 0.0);
        assert!(zoomed.camera.position.y > 0.0);
        assert_eq!(adapter.camera.position, DVec2::ZERO);
    }

    #[test]
    fn plain_zoom_still_keeps_the_screen_center_fixed() {
        let mut adapter = ScrollAdapter::new(500.0, 500.0, None, None, None);
        adapter = adapter.zoom_at(1.0, DVec2::new(500.0, 0.0));
        adapter = adapter.zoom(0.5);

        assert!(adapter.zoom_anchor.is_none());

        let center = adapter.screen_center;
        let before = adapter.unproject(center);

        for _ in 0..200 {
            adapter = adapter.updated(DT);
            assert!((adapter.unproject(center) - before).length() < 1e-9);
        }
    }

    #[test]
    fn set_zoom_at_jumps_to_the_goal_and_still_anchors() {
        let mut adapter = ScrollAdapter::new(500.0, 500.0, None, None, None);
        let anchor = DVec2::new(310.0, 470.0);
        let before = adapter.unproject(anchor);

        let zoomed = settled(adapter.set_zoom_at(2.0, anchor), 200);

        assert!((zoomed.camera.zoom - 2.0).abs() < 1e-9);
        assert!((zoomed.unproject(anchor) - before).length() < 1e-9);
    }
}
