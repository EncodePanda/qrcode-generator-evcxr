use qrcode_generator_evcxr::{draw_qrcode, draw_qrcode_with_config, QrConfig, QrCodeEcc};

#[test]
fn test_draw_qrcode_does_not_panic() {
    // We can't easily capture evcxr protocol output in tests,
    // but we can verify it doesn't panic
    draw_qrcode("https://example.com");
}

#[test]
fn test_draw_qrcode_with_string() {
    let url = String::from("https://example.com");
    draw_qrcode(&url);
}

#[test]
fn test_draw_qrcode_with_config_does_not_panic() {
    let config = QrConfig {
        ec_level: QrCodeEcc::High,
        size: 512,
        margin: 2,
    };
    draw_qrcode_with_config("https://example.com", config);
}

#[test]
fn test_draw_qrcode_with_matrix() {
    let matrix = vec![
        vec![true, false, true],
        vec![false, true, false],
        vec![true, false, true],
    ];
    draw_qrcode(matrix);
}

#[test]
fn test_draw_qrcode_default_config() {
    let config = QrConfig::default();
    assert_eq!(config.size, 256);
    assert_eq!(config.margin, 4);
}
