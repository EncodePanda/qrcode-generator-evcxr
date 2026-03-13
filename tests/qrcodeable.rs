use qrcode_generator_evcxr::{QrCodeEcc, QrCodeable};

#[test]
fn test_str_to_matrix() {
    let matrix = "hello".to_matrix(QrCodeEcc::Medium);
    // QR code matrix should be non-empty and square
    assert!(!matrix.is_empty());
    assert_eq!(matrix.len(), matrix[0].len());
}

#[test]
fn test_string_to_matrix() {
    let input = String::from("hello");
    let matrix = input.to_matrix(QrCodeEcc::Medium);
    assert!(!matrix.is_empty());
    assert_eq!(matrix.len(), matrix[0].len());
}

#[test]
fn test_string_ref_to_matrix() {
    let input = String::from("hello");
    let matrix = (&input).to_matrix(QrCodeEcc::Medium);
    assert!(!matrix.is_empty());
    assert_eq!(matrix.len(), matrix[0].len());
}

#[test]
fn test_vec_vec_bool_to_matrix() {
    let original = vec![
        vec![true, false],
        vec![false, true],
    ];
    let matrix = original.to_matrix(QrCodeEcc::Low);
    // Should return a clone of itself, ignoring ec_level
    assert_eq!(matrix, vec![vec![true, false], vec![false, true]]);
}
