pub mod particle;

pub type SimFloat = f64;

pub mod proto {
    use std::io::Result as IoResult;

    use nalgebra as na;
    use crate::{particle::proto::ParticleProto, SimFloat};
    use serde::{Deserialize, Serialize};

    // TODO: Custom properties
    #[derive(Clone, Copy, Debug, Serialize, Deserialize)]
    pub struct SimulationConfig {
        pub g_const: SimFloat,
    }

    #[derive(Clone, Copy, Debug, Serialize, Deserialize)]
    pub struct RungeKuttaSolverConfig {
        pub timestep: SimFloat,
    }

    #[derive(Clone, Copy, Debug, Serialize, Deserialize)]
    pub struct ParticleDefinition {
        position: [f64; 2],
        velocity: [f64; 2],
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Configuration {
        simulation_config: SimulationConfig,
        solver_config: RungeKuttaSolverConfig,
        initial_objects: Vec<ParticleDefinition>,
    }

    impl Configuration {
        pub fn save(&self, filename: &str) -> IoResult<()> {
            let stringified = serde_json::to_string_pretty(self)?;

            std::fs::write(filename, stringified)
        }
    }

    impl Default for Configuration {
        fn default() -> Self {
            Self {
                simulation_config: SimulationConfig { g_const: 1.0 },
                solver_config: RungeKuttaSolverConfig { timestep: 0.02 },
                initial_objects: vec![]
            }
        }
    }

    // NOTE: This is something user will define/configure
    pub struct ParticleSimulator {
        solver: RungeKuttaSolver,
        sim_config: SimulationConfig,
        objects: Vec<ParticleProto<2>>,
        simulation_time: SimFloat,
    }

    impl ParticleSimulator {
        pub fn load(filename: &str) -> IoResult<Self> {
            let file = std::fs::read_to_string(filename)?;
            let config: Configuration = serde_json::from_str(&file)?;

            Ok(Self {
                solver: RungeKuttaSolver::new(config.solver_config),
                sim_config: config.simulation_config,
                objects: config.initial_objects.into_iter()
                    .map(|p| ParticleProto {
                        position: p.position.into(),
                        velocity: p.velocity.into(),
                    }).collect::<Vec<_>>(),
                simulation_time: 0.0,
            })
        }

        pub fn new() -> Self {
            Self {
                solver: RungeKuttaSolver::new(
                    RungeKuttaSolverConfig { timestep: 0.02 },
                ),
                sim_config: SimulationConfig { g_const: 100.0 },
                objects: vec![],
                simulation_time: 0.0,
            }
        }

        pub fn particles(&self) -> &[ParticleProto<2>] {
            &self.objects
        }

        pub fn step(&mut self) {
            let mut forces = vec![na::SVector::<SimFloat, 2>::zeros(); self.objects.len()];
            for (i, x) in self.objects.iter().enumerate() {
                for (j, y) in self.objects.iter().enumerate() {
                    if i == j { continue }
                    forces[i] += x.gravity(y, self.sim_config.g_const);
                }
            }

            self.solver.step_simulation(&mut self.objects, &forces);

            self.simulation_time += self.solver.delta();
        }

        pub fn time(&self) -> SimFloat {
            self.simulation_time
        }
    }

    pub struct RungeKuttaSolver {
        config: RungeKuttaSolverConfig,
    }

    impl RungeKuttaSolver {
        pub fn new(
            config: RungeKuttaSolverConfig,
        ) -> Self {
            Self {
                config,
            }
        }

        pub fn delta(&self) -> SimFloat {
            self.config.timestep
        }

        pub fn step_simulation<const N: usize, TObj>(
            &self,
            objects: &mut [TObj],
            forces: &[na::SVector<SimFloat, N>]
        ) where TObj: RungeKuttaObject<N> {
            for (obj, force) in std::iter::zip(objects.iter_mut(), forces.iter()) {
                obj.step(force.clone(), self.config.timestep);
            }
        }
    }

    pub trait RungeKuttaObject<const N: usize> {
        fn step(&mut self, force: na::SVector<SimFloat, N>, delta: SimFloat);
    }
}
