#![cfg(target_os = "windows")]

use std::{fs, process::Command, thread, time::Duration};
use water_simulation::REQUIRED_ASSETS;

struct TestProcess(std::process::Child);

impl Drop for TestProcess {
    fn drop(&mut self) {
        if matches!(self.0.try_wait(), Ok(None)) {
            if let Err(error) = self.0.kill() {
                eprintln!("No se pudo cerrar el proceso de prueba: {error}");
            }
            if let Err(error) = self.0.wait() {
                eprintln!("No se pudo esperar al proceso de prueba: {error}");
            }
        }
    }
}

fn title(pid: u32) -> String {
    for _ in 0..60 {
        let output = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                &format!(
                    "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; (Get-Process -Id {pid} -ErrorAction Stop).MainWindowTitle"
                ),
            ])
            .output()
            .expect("consultar la ventana del proceso");
        let title = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        if !title.is_empty() {
            return title;
        }
        thread::sleep(Duration::from_millis(500));
    }
    panic!("el proceso {pid} no abrió una ventana");
}

fn window_text(pid: u32) -> String {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-File"])
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/window_ocr.ps1"))
        .arg("-ProcessId")
        .arg(pid.to_string())
        .output()
        .expect("capturar la ventana real");
    assert!(
        output.status.success(),
        "falló la captura OCR: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn close_window(pid: u32) {
    let status = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            &format!(
                "$p = Get-Process -Id {pid} -ErrorAction Stop; if (-not $p.CloseMainWindow()) {{ exit 1 }}; if (-not $p.WaitForExit(10000)) {{ exit 2 }}"
            ),
        ])
        .status()
        .expect("cerrar la ventana real");
    assert!(status.success(), "la aplicación no se cerró normalmente");
}

#[test]
fn arranque_fallido_y_reapertura_tras_corregir_assets() {
    let dir = tempfile::tempdir().expect("crear directorio temporal");
    let assets = dir.path().join("assets");
    fs::create_dir(&assets).expect("crear directorio de assets");
    for name in REQUIRED_ASSETS {
        image::RgbImage::new(2, 2)
            .save(assets.join(name))
            .expect("crear imagen de prueba");
    }
    fs::remove_file(assets.join("tiles.jpg")).expect("eliminar imagen");
    fs::write(assets.join("xneg.jpg"), b"danado").expect("dañar imagen");

    let mut first = TestProcess(
        Command::new(env!("CARGO_BIN_EXE_water-simulation"))
            .current_dir(dir.path())
            .spawn()
            .expect("iniciar aplicación"),
    );
    let first_title = title(first.0.id());
    let message = window_text(first.0.id());
    close_window(first.0.id());
    first.0.wait().expect("esperar primera instancia");
    assert!(first_title.contains("tiles.jpg"));
    assert!(first_title.contains("xneg.jpg"));
    assert!(!first_title.contains("Simulación iniciada"));
    assert!(message.contains("tiles.jpg"), "mensaje visible: {message}");
    assert!(message.contains("xneg.jpg"), "mensaje visible: {message}");
    assert!(
        message.contains("No se puede iniciar la simulación"),
        "mensaje visible: {message}"
    );

    for name in ["tiles.jpg", "xneg.jpg"] {
        image::RgbImage::new(2, 2)
            .save(assets.join(name))
            .expect("corregir imagen");
    }
    let mut second = TestProcess(
        Command::new(env!("CARGO_BIN_EXE_water-simulation"))
            .current_dir(dir.path())
            .spawn()
            .expect("reabrir aplicación"),
    );
    let second_title = title(second.0.id());
    close_window(second.0.id());
    second.0.wait().expect("esperar segunda instancia");
    assert_eq!(second_title, "Simulación de agua");

    for name in REQUIRED_ASSETS {
        fs::remove_file(assets.join(name)).expect("eliminar asset");
    }
    let mut third = TestProcess(
        Command::new(env!("CARGO_BIN_EXE_water-simulation"))
            .current_dir(dir.path())
            .spawn()
            .expect("iniciar aplicación sin assets"),
    );
    let _ = title(third.0.id());
    let message = window_text(third.0.id());
    close_window(third.0.id());
    third.0.wait().expect("esperar tercera instancia");
    for name in REQUIRED_ASSETS {
        assert!(message.contains(name), "falta {name} en: {message}");
    }
    assert!(
        message.contains("No se puede iniciar la simulación"),
        "mensaje visible: {message}"
    );
}
