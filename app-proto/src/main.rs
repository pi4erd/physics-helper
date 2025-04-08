use simcore::proto::ParticleSimulator;
use visualize::proto::RaylibVisualizer;

#[test]
fn default_config() {
    use simcore::proto::Configuration;
    Configuration::default().save("default.json").unwrap();
}

fn main() {
    let mut raylib_instance = RaylibVisualizer::new();

    let mut engine = ParticleSimulator::load("app-proto/default.json").unwrap();

    engine.start_recording_statistics();
    while raylib_instance.is_looping() {
        raylib_instance.camera_control();

        engine.step();

        raylib_instance.draw_particles(engine.particles());
    }
    engine.save_statistics("stats.csv").unwrap();
}
