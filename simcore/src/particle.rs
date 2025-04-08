



pub mod proto {
    use std::collections::HashMap;
    // NOTE: nalgebra is not the fastest library but it is accurate
    use nalgebra as na;

    use crate::{proto::RungeKuttaObject, SimFloat, Property};

    pub type InteractionFn<const N: usize> = fn(
        p1: &ParticleProto<N>,
        p2: &ParticleProto<N>,
        simulation_properties: &HashMap<String, Property>
    ) -> na::SVector<SimFloat, N>;

    pub struct ParticleProto<const N: usize> {
        pub position: na::Point<SimFloat, N>,
        pub velocity: na::SVector<SimFloat, N>,
    }

    impl<const N: usize> ParticleProto<N> {
        pub fn new() -> Self {
            Self {
                position: na::Point::origin(),
                velocity: na::SVector::zeros(),
            }
        }
    }

    impl<const N: usize> RungeKuttaObject<N> for ParticleProto<N> {
        fn step(&mut self, force: na::SVector<SimFloat, N>, delta: SimFloat) {
            self.velocity += force * delta;
            self.position += self.velocity * delta;
        }
    }
}


