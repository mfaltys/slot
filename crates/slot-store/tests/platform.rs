use slot_store::Platform;

#[test]
fn every_platform_has_a_directory() {
    let names: Vec<&str> = Platform::ALL.iter().map(|p| p.dir_name()).collect();
    assert_eq!(names, vec!["GBA", "GBAV", "GBC", "GB", "PICO8"]);
}

#[test]
fn every_platform_is_a_shelf_of_its_own() {
    assert_eq!(
        Platform::ALL,
        [
            Platform::Gba,
            Platform::Gbav,
            Platform::Gbc,
            Platform::Gb,
            Platform::Pico8
        ]
    );
    for (i, p) in Platform::ALL.iter().enumerate() {
        assert!(
            !Platform::ALL[..i].contains(p),
            "{p:?} is named twice in the ring, so two shelves would hold one platform"
        );
    }
}

#[test]
fn each_platform_takes_only_its_own_extensions() {
    assert!(Platform::Gba.accepts(std::path::Path::new("Metroid Fusion.gba")));
    assert!(!Platform::Gba.accepts(std::path::Path::new("Tetris.gb")));
    assert!(Platform::Gb.accepts(std::path::Path::new("Tetris.gb")));
    assert!(Platform::Gb.accepts(std::path::Path::new("Tetris.gbc")));
    assert!(!Platform::Gb.accepts(std::path::Path::new("Metroid Fusion.gba")));
    assert!(Platform::Gba.accepts(std::path::Path::new("Shrek.GBA")));
    assert!(Platform::Gbav.accepts(std::path::Path::new("Shrek.gba")));
    assert!(!Platform::Gbav.accepts(std::path::Path::new("Tetris.gb")));
}

#[test]
fn pico8_takes_p8_and_png_carts_only() {
    assert!(Platform::Pico8.accepts(std::path::Path::new("Celeste.p8")));
    assert!(Platform::Pico8.accepts(std::path::Path::new("Celeste.p8.png")));
    assert!(Platform::Pico8.accepts(std::path::Path::new("Celeste.PNG")));
    assert!(!Platform::Pico8.accepts(std::path::Path::new("Tetris.gb")));
    assert!(!Platform::Pico8.accepts(std::path::Path::new("Metroid Fusion.gba")));
}

#[test]
fn pico8_is_a_square_picture_on_its_own_hardware() {
    assert_eq!(Platform::Pico8.picture(), (128, 128));
    assert_eq!(Platform::Pico8.hardware(), Platform::Pico8);
    assert_eq!(Platform::Pico8.name(), "PICO-8");
}

#[test]
fn gba_video_runs_on_gba_hardware() {
    assert_eq!(Platform::Gbav.hardware(), Platform::Gba);
    assert_eq!(Platform::Gbav.picture(), Platform::Gba.picture());
    for p in [Platform::Gba, Platform::Gb, Platform::Gbc, Platform::Pico8] {
        assert_eq!(p.hardware(), p);
    }
}
