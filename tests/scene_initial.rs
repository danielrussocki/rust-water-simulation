use water_simulation::scene::Scene;

#[test]
fn initial_scene_matches_reference() {
    let scene = Scene::with_random(|| 0.5);
    assert_eq!(scene.sphere_center, [-0.4, -0.75, 0.2]);
    assert_eq!(scene.sphere_radius, 0.25);
    assert_eq!(scene.sphere_velocity, [0.0, 0.0, 0.0]);
    assert_eq!(scene.camera_angles, [-25.0, -200.5]);
    assert_eq!(scene.light_direction, [2.0 / 3.0, 2.0 / 3.0, -1.0 / 3.0]);
    assert!(!scene.paused);
    assert!(!scene.sphere_physics);
}

#[test]
fn initial_drops_use_independent_uniform_coordinates_and_alternating_strengths() {
    let mut index = 0;
    let scene = Scene::with_random(|| {
        let value = index as f32 / 40.0;
        index += 1;
        value
    });
    assert_eq!(index, 40);
    assert_eq!(scene.initial_drops.len(), 20);
    for (i, drop) in scene.initial_drops.iter().enumerate() {
        assert_eq!(
            drop.position,
            [i as f32 / 10.0 - 1.0, (2 * i + 1) as f32 / 20.0 - 1.0]
        );
        assert_eq!(drop.radius, 0.03);
        assert_eq!(drop.strength, if i % 2 == 0 { -0.01 } else { 0.01 });
    }
}

#[test]
fn initial_drop_coordinates_cover_the_surface_without_coupling_axes() {
    let samples = [0.0, 0.75, 0.25, 0.999_999];
    let mut index = 0;
    let scene = Scene::with_random(|| {
        let value = samples[index % samples.len()];
        index += 1;
        value
    });

    assert_eq!(index, 40);
    for pair in scene.initial_drops.chunks_exact(2) {
        assert_eq!(pair[0].position, [-1.0, 0.5]);
        assert_eq!(pair[1].position, [-0.5, 0.999_998]);
        for drop in pair {
            assert!(
                drop.position
                    .iter()
                    .all(|value| (-1.0..1.0).contains(value))
            );
        }
    }
}
