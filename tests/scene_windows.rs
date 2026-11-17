#![cfg(target_os = "windows")]

use std::process::Command;

#[test]
fn original_assets_produce_a_visible_animated_scene() {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-File"])
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/scene_capture.ps1"
        ))
        .arg(env!("CARGO_BIN_EXE_water-simulation"))
        .arg(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("ejecutar la prueba visual sobre la aplicación real");
    assert!(
        output.status.success(),
        "la escena no se verificó: {} {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
