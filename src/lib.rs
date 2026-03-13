pub use qrcode_generator::QrCodeEcc;

/// Configuration for QR code rendering.
pub struct QrConfig {
    /// Error correction level.
    pub ec_level: QrCodeEcc,
    /// Total SVG size in pixels (width and height).
    pub size: usize,
    /// Quiet zone margin in modules.
    pub margin: usize,
}

impl Default for QrConfig {
    fn default() -> Self {
        QrConfig {
            ec_level: QrCodeEcc::Medium,
            size: 256,
            margin: 4,
        }
    }
}
