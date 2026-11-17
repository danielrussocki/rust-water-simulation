use water_simulation::scene::{Scene, Water};

#[test]
fn scene_updates_water_only_for_accepted_active_intervals() {
    let mut scene = Scene::with_random(|| 0.5);
    scene.water.set_cell(128, 128, 1.0, 0.0);
    let initial = scene.water.cell(128, 128);
    scene.update(1.001);
    assert_eq!(scene.water.cell(128, 128), initial);
    scene.update(0.016);
    assert_ne!(scene.water.cell(128, 128), initial);
}

#[test]
fn an_update_applies_two_reference_steps_with_damping() {
    let mut water = Water::new(3);
    water.set_cell(1, 1, 1.0, 0.0);
    water.update(0.016);
    // First step: center velocity -1.99, height -0.99; neighbors height 0.4975.
    // Second step uses the newly updated neighbors and damps velocity again.
    let (height, velocity) = water.cell(1, 1);
    assert!((velocity - 0.980_075).abs() < 0.000_01);
    assert!((height + 0.009_925).abs() < 0.000_01);
}

#[test]
fn an_interval_greater_than_one_second_is_skipped() {
    let mut water = Water::new(3);
    water.set_cell(1, 1, 1.0, 0.25);
    water.update(1.001);
    assert_eq!(water.cell(1, 1), (1.0, 0.25));
}

#[test]
fn an_interval_of_exactly_one_second_is_not_skipped() {
    let mut water = Water::new(3);
    water.set_cell(1, 1, 1.0, 0.25);
    water.update(1.0);
    assert_ne!(water.cell(1, 1), (1.0, 0.25));
}
