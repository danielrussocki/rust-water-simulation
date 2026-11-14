use std::fs;

use water_simulation::{REQUIRED_ASSETS, StartupDecision, startup_decision, validate_assets};

fn valid_assets() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("crear directorio temporal para la prueba");
    for name in REQUIRED_ASSETS {
        image::RgbImage::new(2, 2)
            .save(dir.path().join(name))
            .expect("crear imagen JPEG válida para la prueba");
    }
    dir
}

#[test]
fn un_asset_ausente_impide_el_inicio_y_se_identifica() {
    let dir = valid_assets();
    fs::remove_file(dir.path().join("tiles.jpg")).expect("eliminar el asset de prueba");
    let errors = validate_assets(dir.path());
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].name, "tiles.jpg");
    assert!(matches!(
        startup_decision(dir.path()),
        StartupDecision::Blocked(_)
    ));
}

#[test]
fn enumera_todos_los_assets_ausentes_y_danados() {
    let dir = valid_assets();
    fs::remove_file(dir.path().join("tiles.jpg")).expect("eliminar el asset de prueba");
    fs::remove_file(dir.path().join("xneg.jpg")).expect("eliminar el asset de prueba");
    fs::write(dir.path().join("zpos.jpg"), b"imagen danada").expect("dañar el asset de prueba");
    let errors = validate_assets(dir.path());
    let names: Vec<_> = errors.iter().map(|error| error.name).collect();
    assert_eq!(names, ["tiles.jpg", "xneg.jpg", "zpos.jpg"]);
    match startup_decision(dir.path()) {
        StartupDecision::Blocked(errors) => assert_eq!(
            errors.iter().map(|error| error.name).collect::<Vec<_>>(),
            ["tiles.jpg", "xneg.jpg", "zpos.jpg"]
        ),
        StartupDecision::Ready => panic!("se permitió iniciar con recursos inválidos"),
    }
}

#[test]
fn reparar_los_assets_permita_un_nuevo_intento() {
    let dir = valid_assets();
    fs::write(dir.path().join("ypos.jpg"), b"imagen danada").expect("dañar el asset de prueba");
    assert_eq!(validate_assets(dir.path()).len(), 1);
    match startup_decision(dir.path()) {
        StartupDecision::Blocked(errors) => {
            assert_eq!(errors.len(), 1);
            assert_eq!(errors[0].name, "ypos.jpg");
        }
        StartupDecision::Ready => panic!("se permitió iniciar con un recurso dañado"),
    }
    image::RgbImage::new(2, 2)
        .save(dir.path().join("ypos.jpg"))
        .expect("reparar el asset de prueba");
    assert!(validate_assets(dir.path()).is_empty());
    assert!(matches!(
        startup_decision(dir.path()),
        StartupDecision::Ready
    ));
}
