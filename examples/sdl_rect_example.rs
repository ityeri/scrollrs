use clock::Clock;
use glam::DVec2;
use scrollrs::{self, Projector};
use sdl2::{
    event::{Event, WindowEvent},
    mouse::MouseWheelDirection,
    pixels::Color,
    rect::Rect,
};

fn main() {
    let mut screen_width: u32 = 500;
    let mut screen_height: u32 = 500;

    let fps: f64 = 60.0;
    let mut clock = Clock::new(1.0 / fps);
    let mut adaptor =
        scrollrs::ScrollAdaptor::new(screen_width as f64, screen_height as f64, None, None, None);
    let wheel_sensitivity = 0.02;

    let rect_position = DVec2::new(-3.0, 3.0);
    let rect_size = DVec2::new(6.0, -6.0);

    let sdl = sdl2::init().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window("wasans", screen_width, screen_height)
        .position_centered()
        .resizable()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    'running: loop {
        let dt = clock.last_dt;

        let mouse_state = event_pump.mouse_state();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::Window { win_event, .. } => {
                    if let WindowEvent::Resized(width, height) = win_event {
                        screen_width = width as u32;
                        screen_height = height as u32;
                        adaptor =
                            adaptor.resized(screen_width as f64, screen_height as f64, None, None);
                    }
                }
                Event::MouseMotion {
                    x, y, xrel, yrel, ..
                } => {
                    if mouse_state.left() {
                        let camera_delta =
                            adaptor.unproject_scalev(DVec2::new(xrel as f64, yrel as f64));
                        adaptor = adaptor.moved(-camera_delta);
                    }
                }
                Event::MouseWheel {
                    x, y, direction, ..
                } => {
                    let amount = match direction {
                        MouseWheelDirection::Normal => y,
                        MouseWheelDirection::Flipped => -y,
                        _ => 0,
                    };
                    adaptor = adaptor.zoom(amount as f64 * wheel_sensitivity);
                }

                _ => {}
            }
        }

        let projected_position = adaptor.project(rect_position);
        let projected_size = adaptor.project_scalev(rect_size);

        let rect = Rect::new(
            projected_position.x as i32,
            projected_position.y as i32,
            projected_size.x as u32,
            projected_size.y as u32,
        );

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 0, 255));
        canvas.fill_rect(rect).unwrap();

        adaptor = adaptor.updated(dt);

        canvas.present();
        clock = clock.tick(fps);
    }
}
