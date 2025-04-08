use serde::{Deserialize, Serialize};

pub mod particle;
pub mod stats;

pub type SimFloat = f64;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Property {
    Float(SimFloat),
    Vector2([SimFloat; 2]),
    Vector3([SimFloat; 3]),
    Vector4([SimFloat; 4]),
}

pub mod proto {
    use std::{collections::HashMap, io::Result as IoResult};

    use nalgebra as na;
    use crate::{particle::proto::{InteractionFn, ParticleProto}, stats::Timeseries, Property, SimFloat};
    use serde::{Deserialize, Serialize};

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
        simulation_config: HashMap<String, Property>,
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
                simulation_config: HashMap::new(),
                solver_config: RungeKuttaSolverConfig { timestep: 0.02 },
                initial_objects: vec![]
            }
        }
    }

    pub struct ParticleSimulator {
        solver: RungeKuttaSolver,
        sim_config: HashMap<String, Property>,
        objects: Vec<ParticleProto<2>>,
        simulation_time: SimFloat,
        interaction_fn: InteractionFn<2>,
        stats: Option<Timeseries<HashMap<String, Property>>>,
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
                
                // TODO: Define way to implement interaction fn
                interaction_fn: |p1, p2, options| {
                    // simulate gravity interaction for prototype
                    let h = p2.position - p1.position;
                    let dst = h.magnitude();
                    let dir = h / dst;

                    // F = Gm1m2/r^2;
                    if let Property::Float(g_const) = options["g_const"] {
                        return g_const * dir / (dst * dst);
                    } else {
                        // TODO: better decoding for options by using wrapper type
                        panic!("Invalid property type for 'g_const'")
                    }
                },
                stats: None,
            })
        }

        pub fn new() -> Self {
            Self {
                solver: RungeKuttaSolver::new(
                    RungeKuttaSolverConfig { timestep: 0.02 },
                ),
                sim_config: HashMap::new(),
                objects: vec![],
                simulation_time: 0.0,
                interaction_fn: |_, _, _| { na::SVector::zeros() },
                stats: None,
            }
        }

        // TODO: Add a way to define which statistics to record and how
        pub fn start_recording_statistics(&mut self) {
            // Using classical Newtonian physics
            // Ek = mv^2 / 2 = v^2 / 2, assuming mass is equal

            self.stats = Some(Timeseries::new());
        }

        pub fn save_statistics(&self, filename: &str) -> IoResult<()> {
            if let Some(stats) = self.stats.as_ref() {
                stats.save(filename)?
            }

            Ok(())
        }

        pub fn particles(&self) -> &[ParticleProto<2>] {
            &self.objects
        }

        pub fn step(&mut self) {
            let particles = self.particles();
            let mut kinetic_energy = 0.0;
            for particle in particles {
                kinetic_energy += particle.velocity.magnitude_squared() / 2.0;
            }

            let mut potential_energy = 0.0;
            for (i, x) in particles.iter().enumerate() {
                for (j, y) in particles.iter().enumerate() {
                    if i == j { continue }
                    potential_energy += (x.position - y.position).magnitude();
                }
            }

            if let Some(stats) = self.stats.as_mut() {
                let mut hashmap = HashMap::new();
                hashmap.insert("kinetic_energy".to_string(), Property::Float(kinetic_energy));
                hashmap.insert("potential_energy".to_string(), Property::Float(potential_energy));
                stats.record(hashmap);
            }

            let mut forces = vec![na::SVector::<SimFloat, 2>::zeros(); self.objects.len()];
            for (i, x) in self.objects.iter().enumerate() {
                for (j, y) in self.objects.iter().enumerate() {
                    if i == j { continue }
                    let force = (self.interaction_fn)(x, y, &self.sim_config);
                    forces[i] += force;
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
