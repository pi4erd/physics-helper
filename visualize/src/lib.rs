pub mod proto {
    use raylib::prelude::*;
    use simcore::particle::proto::ParticleProto;

    #[derive(Clone, Copy, Debug)]
    pub struct Camera {
        position: Vector2,
        zoom: f32,
    }

    pub struct DrawHandle<'a> {
        handle: RaylibDrawHandle<'a>,
        pub camera: Camera,
    }

    #[derive(Clone, Copy, Debug)]
    pub enum Position<T: Clone + Copy> {
        World(T, T),
        View(T, T),
    }

    impl DrawHandle<'_> {
        pub fn is_dragging(&self) -> bool {
            self.handle.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT)
        }

        pub fn text(&mut self, position: Position<i32>, text: &str) {
            match position {
                Position::View(x, y) => {
                    self.handle.draw_text(text, x, y, 20, Color::WHITE);
                }
                Position::World(_x, _y) => {
                    todo!();
                    // self.handle.draw_text(
                    //     text,
                    //     (x as f32 - self.camera.position.x) / self.camera.zoom,
                    //     (y as f32 - self.camera.position.y) / self.camera.zoom,
                    //     20,
                    //     Color::WHITE,
                    // );
                }
            }
        }

        fn point(&mut self, position: Position<f32>) {
            match position {
                Position::View(x, y) => {
                    self.handle.draw_circle_v(Vector2::new(x, y), 10.0, Color::WHITE);
                }
                Position::World(x, y) => {
                    let resolution = Vector2::new(
                        self.handle.get_screen_width() as f32,
                        self.handle.get_screen_height() as f32,
                    );
                    self.handle.draw_circle_v(
                        (Vector2::new(x, y) - self.camera.position) / self.camera.zoom
                            + resolution / 2.0,
                        (2.0 / self.camera.zoom).max(1.0),
                        Color::WHITE,
                    );
                }
            }
        }
    }

    pub struct RaylibVisualizer {
        raylib: RaylibHandle,
        thread: RaylibThread,
        camera: Camera,
        drag: Option<(Vector2, Vector2)>,
    }

    impl RaylibVisualizer {
        pub fn new() -> Self {
            let (rl, thread) = raylib::init()
                .size(1280, 720)
                .build();

            Self {
                raylib: rl,
                thread,
                camera: Camera {
                    position: Vector2::zero(),
                    zoom: 1.0,
                },
                drag: None,
            }
        }

        fn begin_draw(&mut self, camera: Camera) -> DrawHandle {
            DrawHandle {
                handle: self.raylib.begin_drawing(&self.thread),
                camera,
            }
        }

        pub fn camera_control(&mut self) {
            if self.raylib.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                self.drag = Some((self.raylib.get_mouse_position(), self.camera.position));
            } else if self.raylib.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                self.drag = None;
            }

            if let Some((initial, camera_initial)) = self.drag {
                let mouse_position = self.raylib.get_mouse_position();
                self.camera.position = (initial - mouse_position) * self.camera.zoom + camera_initial;
            }

            let mouse_wheel = self.raylib.get_mouse_wheel_move_v();

            self.camera.zoom *= 1.0 - (mouse_wheel.y * 0.1).clamp(-3.0, 3.0);
        }

        pub fn is_looping(&self) -> bool {
            !self.raylib.window_should_close()
        }

        pub fn draw_particles(&mut self, particles: &[ParticleProto<2>], time: f64) {
            let mut draw = self.begin_draw(self.camera);
            draw.handle.clear_background(Color::BLACK);

            for particle in particles.iter() {
                draw.point(Position::World(
                    particle.position.x as f32,
                    particle.position.y as f32,
                ));
            }

            draw.text(
                Position::View(10, 10),
                &format!("Particle count: {}", particles.len())
            );
            draw.text(
                Position::View(10, 40),
                &format!("Current simulation type: Particle")
            );
            draw.text(
                Position::View(10, 70),
                &format!("Current simulation time: {time}")
            );
        }
    }
}

