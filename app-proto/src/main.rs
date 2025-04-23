use simcore::proto::ParticleSimulator;
use visualize::proto::RaylibVisualizer;

#[test]
fn default_config() {
    use simcore::proto::Configuration;
    Configuration::default().save("default.json").unwrap();
}

fn main() {
    let mut raylib_instance = RaylibVisualizer::new();

    // request for configuration
    let config_file = rfd::FileDialog::new()
        .add_filter("json", &["json"])
        .set_title("Select configuration file")
        .set_directory("/")
        .pick_file();

    let config_file = if let Some(c) = config_file { c } else {
        eprintln!("No config file chosen!");
        return;
    };

    let config_file = config_file.to_str()
        .expect("Unable to parse config path as utf-8 string");

    let mut engine = match ParticleSimulator::load(config_file) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error occured while loading configuration: {e}");
            return;
        }
    };

    engine.start_recording_statistics();
    while raylib_instance.is_looping() {
        raylib_instance.camera_control();

        engine.step();

        raylib_instance.draw_particles(engine.particles(), engine.sim_name(), engine.time());
    }
    engine.save_statistics("stats.json").unwrap();
}
