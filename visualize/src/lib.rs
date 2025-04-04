pub mod proto {
    use raylib::prelude::*;

    pub struct RaylibInstance {
        raylib: RaylibHandle,
        thread: RaylibThread,
    }

    impl RaylibInstance {
        pub fn new() -> Self {
            let (rl, thread) = raylib::init()
                .size(1280, 720)
                .build();

            Self {
                raylib: rl,
                thread,
            }
        }

        pub fn raylib(&mut self) -> &mut RaylibHandle {
            &mut self.raylib
        }

        pub fn thread(&self) -> &RaylibThread {
            &self.thread
        }

        pub fn begin_draw(&mut self) -> RaylibDrawHandle {
            let mut draw = self.raylib.begin_drawing(&self.thread);
            draw.clear_background(Color::BLACK);
            draw
        }

        pub fn should_loop(&self) -> bool {
            !self.raylib.window_should_close()
        }

        pub fn text(&self, draw: &mut RaylibDrawHandle, txt: &str) {
            draw.draw_text(txt, 0, 0, 16, Color::WHITE);
        }
    }

    pub fn point(draw: &mut RaylibDrawHandle, x: f32, y: f32) {
        draw.draw_circle_v(Vector2::new(x, y), 2.0, Color::WHITE);
    }
}

