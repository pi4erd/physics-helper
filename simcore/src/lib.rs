pub mod particle;

pub type SimFloat = f64;

pub mod proto {
    use std::{marker::PhantomData, slice::Iter};

    use crate::{particle::proto::{ParticleInteraction, ParticleProto}, SimFloat};
    use nalgebra as na;

    // TODO: Custom properties
    #[derive(Clone, Copy, Debug)]
    pub struct SimulationConfig {
        pub g_const: SimFloat,
    }

    // NOTE: This is something user will define/configure
    pub struct ParticleSimulator {
        solver: RungeKuttaSolver<2, ParticleProto<2>>,
        sim_config: SimulationConfig,
        objects: Vec<ParticleProto<2>>,
        simulation_time: SimFloat,
    }

    impl ParticleSimulator {
        pub fn new() -> Self {
            Self {
                solver: RungeKuttaSolver::new(
                    RungeKuttaSolverConfig { timestep: 0.02 },
                ),
                sim_config: SimulationConfig { g_const: 100.0 },
                objects: vec![
                    ParticleProto {
                        position: na::Point2::new(500.0, 400.0),
                        velocity: na::Vector2::new(0.0, 0.4),
                    },
                    ParticleProto {
                        position: na::Point2::new(600.0, 500.0),
                        velocity: na::Vector2::new(0.0, -0.4),
                    }
                ],
                simulation_time: 0.0,
            }
        }

        pub fn particle_iter(&self) -> Iter<ParticleProto<2>> {
            self.objects.iter()
        }

        pub fn step(&mut self) {
            let mut interactions = vec![ParticleInteraction::<2>::default(); self.objects.len()];
            for (i, x) in self.objects.iter().enumerate() {
                for (j, y) in self.objects.iter().enumerate() {
                    if i == j { continue }
                    interactions[i] = x.interaction(&y, &self.sim_config);
                }
            }

            self.solver.step_simulation(&mut self.objects, &interactions);

            self.simulation_time += self.solver.delta();
        }

        pub fn time(&self) -> SimFloat {
            self.simulation_time
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct RungeKuttaSolverConfig {
        pub timestep: SimFloat,
    }

    pub struct RungeKuttaSolver<const N: usize, TObj: RungeKuttaObject<N>> {
        config: RungeKuttaSolverConfig,
        phantom: PhantomData<TObj>,
    }

    impl<const N: usize, TObj: RungeKuttaObject<N>> RungeKuttaSolver<N, TObj> {
        pub fn new(
            config: RungeKuttaSolverConfig,
        ) -> Self {
            Self {
                config,
                phantom: PhantomData {},
            }
        }

        pub fn delta(&self) -> SimFloat {
            self.config.timestep
        }

        pub fn step_simulation(&self, objects: &mut [TObj], interactions: &[ParticleInteraction<N>]) {
            for (obj, interaction) in std::iter::zip(objects.iter_mut(), interactions.iter()) {
                obj.step(interaction.clone(), self.config.timestep);
            }
        }
    }

    pub trait RungeKuttaObject<const N: usize> {
        fn step(&mut self, interaction: ParticleInteraction<N>, delta: SimFloat);
    }
}
