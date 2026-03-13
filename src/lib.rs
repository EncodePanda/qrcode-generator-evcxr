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

/// Render a QR code bool matrix as an SVG string.
///
/// - `matrix`: 2D bool grid where `true` = black module
/// - `size`: total SVG width/height in pixels
/// - `margin`: quiet zone in modules around the QR data
pub fn render_svg(matrix: &[Vec<bool>], size: usize, margin: usize) -> String {
    let data_modules = matrix.len();
    let total_modules = data_modules + 2 * margin;

    if total_modules == 0 {
        return format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{size}\" height=\"{size}\"></svg>"
        );
    }

    let cell_size = size as f64 / total_modules as f64;

    let mut svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{size}\" height=\"{size}\" \
         viewBox=\"0 0 {size} {size}\">\
         <rect width=\"{size}\" height=\"{size}\" fill=\"white\"/>"
    );

    for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell {
                let x = ((col_idx + margin) as f64 * cell_size).round() as usize;
                let y = ((row_idx + margin) as f64 * cell_size).round() as usize;
                let w = (((col_idx + margin + 1) as f64 * cell_size).round() as usize) - x;
                let h = (((row_idx + margin + 1) as f64 * cell_size).round() as usize) - y;
                svg.push_str(&format!(
                    "<rect x=\"{x}\" y=\"{y}\" width=\"{w}\" height=\"{h}\" fill=\"black\"/>"
                ));
            }
        }
    }

    svg.push_str("</svg>");
    svg
}
