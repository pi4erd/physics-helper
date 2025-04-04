use simcore::proto::ParticleSimulator;
use visualize::proto::RaylibInstance;

fn main() {
    let mut raylib_instance = RaylibInstance::new();

    let mut engine = ParticleSimulator::new();

    while raylib_instance.should_loop() {
        engine.step();

        let mut draw = raylib_instance.begin_draw();
        for particle in engine.particle_iter() {
            visualize::proto::point(
                &mut draw,
                particle.position.x as f32,
                particle.position.y as f32,
            );
        }
    }
}
