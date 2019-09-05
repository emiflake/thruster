#[macro_use]
extern crate criterion;

use thruster::scene::Scene;
use thruster::texture_map;

use criterion::Criterion;
fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Screenshot Renders");

    group.sample_size(10);

    group.bench_function("Basic Scene Screenshot 1080p", |b| {
        b.iter_with_setup(
            || {
                let mut texture_map = texture_map::TextureMap::new();

                let scn_str = std::fs::read_to_string("basic_scn.ron")
                    .expect("Could not read configuration file");
                let scene: Scene = ron::de::from_str(&scn_str).expect("Could not parse");
                texture_map.preload_all_in_scene(&scene);

                (scene, texture_map)
            },
            |(scn, tex_map)| {
                scn.screenshot(
                    "tmp_scn1080p.png",
                    f64::from(1920),
                    f64::from(1080),
                    &tex_map,
                )
            },
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
