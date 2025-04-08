



pub mod proto {
    // NOTE: nalgebra is not the fastest library but it is accurate
    use nalgebra as na;
    use visualize::proto::ParticleVisual;

    use crate::{proto::RungeKuttaObject, SimFloat};

    pub struct ParticleProto<const N: usize> {
        pub position: na::Point<SimFloat, N>,
        pub velocity: na::SVector<SimFloat, N>,
    }

    impl<const N: usize> ParticleVisual for ParticleProto<N> {
        fn position(&self) -> visualize::proto::Position<f32> {
            // TODO: 2d/3d distinction
            visualize::proto::Position::World(self.position[0] as f32, self.position[1] as f32)
        }
    }

    impl<const N: usize> ParticleProto<N> {
        pub fn new() -> Self {
            Self {
                position: na::Point::origin(),
                velocity: na::SVector::zeros(),
            }
        }

        // TODO: These must be defined by the user or preset
        pub fn gravity(&self, other: &ParticleProto<N>, g_const: SimFloat) -> na::SVector<SimFloat, N> {
            // simulate gravity interaction for prototype
            let h = other.position - self.position;
            let dst = h.magnitude();
            let dir = h / dst;

            // F = Gm1m2/r^2;
            let force = g_const * dir / (dst * dst);

            force
        }
    }

    impl<const N: usize> RungeKuttaObject<N> for ParticleProto<N> {
        fn step(&mut self, force: na::SVector<SimFloat, N>, delta: SimFloat) {
            self.velocity += force * delta;
            self.position += self.velocity * delta;
        }
    }
}


