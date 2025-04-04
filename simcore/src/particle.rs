



pub mod proto {
    // NOTE: nalgebra is not the fastest library but it is accurate
    use nalgebra as na;

    use crate::{proto::{RungeKuttaObject, SimulationConfig}, SimFloat};

    #[derive(Clone, Debug)]
    pub struct ParticleInteraction<const N: usize> {
        position_change: na::SVector<SimFloat, N>,
        velocity_change: na::SVector<SimFloat, N>,
    }

    impl<const N: usize> Default for ParticleInteraction<N> {
        fn default() -> Self {
            Self {
                position_change: na::SVector::zeros(),
                velocity_change: na::SVector::zeros(),
            }
        }
    }

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

        // TODO: These must be defined by the user or preset
        fn gravity(&self, other: &ParticleProto<N>, g_const: SimFloat) -> na::SVector<SimFloat, N> {
            // simulate gravity interaction for prototype
            let h = other.position - self.position;
            let sqr_dst = h.magnitude_squared();

            // F = Gm1m2/r^2; vF = d * F; d = h/l; G=1
            let force = g_const * h / (sqr_dst * sqr_dst.sqrt());

            force
        }

        // TODO: This must be defined in trait and called from simulator
        pub fn interaction(&self, other: &ParticleProto<N>, config: &SimulationConfig) -> self::ParticleInteraction<N> {
            ParticleInteraction {
                position_change: self.velocity,
                velocity_change: self.gravity(other, config.g_const),
            }
        }
    }

    impl<const N: usize> RungeKuttaObject<N> for ParticleProto<N> {
        fn step(&mut self, interaction: ParticleInteraction<N>, delta: SimFloat) {
            self.position += interaction.position_change * delta;
            self.velocity += interaction.velocity_change * delta;
        }
    }
}


