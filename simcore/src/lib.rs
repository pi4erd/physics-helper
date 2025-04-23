use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod particle;
pub mod stats;

pub type SimFloat = f64;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Property {
    Float(SimFloat),
    Vector2([SimFloat; 2]),
    Vector3([SimFloat; 3]),
    Vector4([SimFloat; 4]),
    String(String),
    Nested(HashMap<String, Property>),
}

impl Property {
    /// Unwrap property as Float. Panics if property wasn't Float
    pub fn float(&self) -> SimFloat {
        match self {
            Property::Float(v) => *v,
            _ => panic!("Unwrap Float on incompatible value: {:?}", self)
        }
    }

    /// Unwrap property as Vector2. Panics if property wasn't Vector2
    pub fn vec2(&self) -> [SimFloat; 2] {
        match self {
            Property::Vector2(v) => *v,
            _ => panic!("Unwrap Vector2 on incompatible value: {:?}", self)
        }
    }

    /// Unwrap property as Vector3. Panics if property wasn't Vector3
    pub fn vec3(&self) -> [SimFloat; 3] {
        match self {
            Property::Vector3(v) => *v,
            _ => panic!("Unwrap Vector3 on incompatible value: {:?}", self)
        }
    }

    /// Unwrap property as Vector4. Panics if property wasn't Vector4
    pub fn vec4(&self) -> [SimFloat; 4] {
        match self {
            Property::Vector4(v) => *v,
            _ => panic!("Unwrap Vector4 on incompatible value: {:?}", self)
        }
    }

    /// Unwrap property as String. Panics if property wasn't String
    pub fn str(&self) -> &str {
        match self {
            Property::String(v) => v,
            _ => panic!("Unwrap String on incompatible value: {:?}", self),
        }
    }

    /// Unwrap property as Nested. Panics if property wasn't Nested
    pub fn nested(&self) -> &HashMap<String, Property> {
        match self {
            Property::Nested(n) => n,
            _ => panic!("Unwrap Nested on incompatible value: {:?}", self),
        }
    }

    /// Unwrap property as Float. Returns None if property wasn't Float
    pub fn try_float(&self) -> Option<SimFloat> {
        match self {
            Property::Float(v) => Some(*v),
            _ => None
        }
    }

    /// Unwrap property as Vector2. Returns None if property wasn't Vector2
    pub fn try_vec2(&self) -> Option<[SimFloat; 2]> {
        match self {
            Property::Vector2(v) => Some(*v),
            _ => None
        }
    }

    /// Unwrap property as Vector3. Returns None if property wasn't Vector3
    pub fn try_vec3(&self) -> Option<[SimFloat; 3]> {
        match self {
            Property::Vector3(v) => Some(*v),
            _ => None
        }
    }

    /// Unwrap property as Vector4. Returns None if property wasn't Vector4
    pub fn try_vec4(&self) -> Option<[SimFloat; 4]> {
        match self {
            Property::Vector4(v) => Some(*v),
            _ => None
        }
    }

    /// Unwrap property as String. Returns None if property wasn't String
    pub fn try_str(&self) -> Option<&str> {
        match self {
            Property::String(v) => Some(v),
            _ => None,
        }
    }

    /// Unwrap property as Nested. Returns None if property wasn't Nested
    pub fn try_nested(&self) -> Option<&HashMap<String, Property>> {
        match self {
            Property::Nested(n) => Some(n),
            _ => None,
        }
    }
}

pub mod proto {
    use std::{collections::HashMap, io::Result as IoResult};

    use nalgebra as na;
    use crate::{particle::proto::{InteractionFn, ParticleProto}, stats::Timeseries, Property, SimFloat};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Copy, Debug, Serialize, Deserialize)]
    pub struct EulerMethodSolverConfig {
        pub timestep: SimFloat,
    }

    pub type ParticleDefinition = HashMap<String, Property>;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Configuration {
        simulation_config: HashMap<String, Property>,
        solver_config: EulerMethodSolverConfig,
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
                solver_config: EulerMethodSolverConfig { timestep: 0.02 },
                initial_objects: vec![]
            }
        }
    }

    pub struct ParticleSimulator {
        solver: EulerMethodSolver,
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
                solver: EulerMethodSolver::new(config.solver_config),
                sim_config: config.simulation_config,
                objects: config.initial_objects.into_iter()
                    .map(|mut p| ParticleProto {
                        position: p.remove("position").unwrap().vec2().into(),
                        velocity: p.remove("velocity").unwrap().vec2().into(),
                        additional_properties: p,
                    }).collect::<Vec<_>>(),
                simulation_time: 0.0,
                
                // TODO: Definable
                interaction_fn: |p1, p2, options| {
                    // simulate gravity interaction for prototype
                    let h = p2.position - p1.position;
                    let dst = h.magnitude();
                    let dir = h / dst;

                    let m1 = if let Some(m) = p1.additional_properties.get("mass") {
                        m.float()
                    } else { 1.0 };

                    let m2 = if let Some(m) = p2.additional_properties.get("mass") {
                        m.float()
                    } else { 1.0 };

                    let g_const = options["g_const"].float();
                    return g_const * m1 * m2 * dir / (dst * dst);
                },
                stats: None,
            })
        }

        pub fn new() -> Self {
            Self {
                solver: EulerMethodSolver::new(
                    EulerMethodSolverConfig { timestep: 0.02 },
                ),
                sim_config: HashMap::new(),
                objects: vec![],
                simulation_time: 0.0,
                interaction_fn: |_, _, _| { na::SVector::zeros() },
                stats: None,
            }
        }

        // TODO: Definable
        pub fn start_recording_statistics(&mut self) {
            self.stats = Some(Timeseries::new());
        }

        pub fn save_statistics(&self, filename: &str) -> IoResult<()> {
            if let Some(stats) = self.stats.as_ref() {
                stats.save(filename)?
            }

            Ok(())
        }

        pub fn sim_name(&self) -> &str {
            self.sim_config["name"].str()
        }

        pub fn particles(&self) -> &[ParticleProto<2>] {
            &self.objects
        }

        // TODO: Definable
        fn record_stats(&mut self) {
            if self.stats.is_none() { return; }
            let particles = self.particles();

            // kinetic = mv^2 / 2
            // potential = -Gm1m2 / r
            let mut kinetic_energy = 0.0;
            let mut potential_energy = 0.0;
            for (i, x) in particles.iter().enumerate() {
                for (j, y) in particles.iter().enumerate() {
                    if i == j { continue }

                    let g_const = self.sim_config["g_const"].float();

                    let m1 = if let Some(m) = x.additional_properties.get("mass") {
                        m.float()
                    } else { 1.0 };

                    let m2 = if let Some(m) = y.additional_properties.get("mass") {
                        m.float()
                    } else { 1.0 };

                    let r = (x.position - y.position).magnitude();
                    potential_energy -= g_const * m1 * m2 / r;

                    // NOTE: Relative kinetic energy. Can it really be aggregated?
                    let k = m1 * (x.velocity - y.velocity).magnitude_squared();
                    kinetic_energy += k;
                }
            }

            let mut hashmap = HashMap::new();
            hashmap.insert("kinetic_energy".to_string(), Property::Float(kinetic_energy));
            hashmap.insert("potential_energy".to_string(), Property::Float(potential_energy));

            for (i, obj) in particles.iter().enumerate() {
                let name = if let Some(name) = obj.additional_properties.get("name") {
                    name.str().to_string()
                } else { i.to_string() };

                let mut obj_props = HashMap::new();
                obj_props.insert("position".to_string(), Property::Vector2(obj.position.into()));
                obj_props.insert("velocity".to_string(), Property::Vector2(obj.position.into()));

                hashmap.insert(format!("{}", name), Property::Nested(obj_props));
            }

            self.stats.as_mut().unwrap().record(hashmap, Some(self.simulation_time));
        }

        // Try and experiment with dynamic delta?
        pub fn step(&mut self) {
            self.record_stats();

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

    pub struct EulerMethodSolver {
        config: EulerMethodSolverConfig,
    }

    impl EulerMethodSolver {
        pub fn new(
            config: EulerMethodSolverConfig,
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
        ) where TObj: EulerMethodObject<N> {
            for (obj, force) in std::iter::zip(objects.iter_mut(), forces.iter()) {
                obj.step(force.clone(), self.config.timestep);
            }
        }
    }

    pub trait EulerMethodObject<const N: usize> {
        fn step(&mut self, force: na::SVector<SimFloat, N>, delta: SimFloat);
    }
}
