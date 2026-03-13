pub use qrcode_generator::QrCodeEcc;

/// Trait for types that can be converted into a QR code matrix.
pub trait QrCodeable {
    fn to_matrix(&self, ec_level: QrCodeEcc) -> Vec<Vec<bool>>;
}

impl QrCodeable for str {
    fn to_matrix(&self, ec_level: QrCodeEcc) -> Vec<Vec<bool>> {
        qrcode_generator::to_matrix_from_str(self, ec_level)
            .expect("Failed to generate QR code matrix")
    }
}

impl QrCodeable for String {
    fn to_matrix(&self, ec_level: QrCodeEcc) -> Vec<Vec<bool>> {
        qrcode_generator::to_matrix_from_str(self, ec_level)
            .expect("Failed to generate QR code matrix")
    }
}

impl QrCodeable for Vec<Vec<bool>> {
    fn to_matrix(&self, _ec_level: QrCodeEcc) -> Vec<Vec<bool>> {
        self.clone()
    }
}

/// Blanket impl so that `&str`, `&String`, `&&str`, etc. all work.
impl<T: QrCodeable + ?Sized> QrCodeable for &T {
    fn to_matrix(&self, ec_level: QrCodeEcc) -> Vec<Vec<bool>> {
        (**self).to_matrix(ec_level)
    }
}

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

/// Display a QR code in Jupyter using default settings.
///
/// Accepts any type implementing `QrCodeable`: `&str`, `String`, or `Vec<Vec<bool>>`.
pub fn draw_qrcode(input: impl QrCodeable) {
    draw_qrcode_with_config(input, QrConfig::default());
}

/// Display a QR code in Jupyter with custom configuration.
///
/// Accepts any type implementing `QrCodeable`: `&str`, `String`, or `Vec<Vec<bool>>`.
pub fn draw_qrcode_with_config(input: impl QrCodeable, config: QrConfig) {
    let matrix = input.to_matrix(config.ec_level);
    let svg = render_svg(&matrix, config.size, config.margin);
    println!("EVCXR_BEGIN_CONTENT image/svg+xml");
    println!("{}", svg);
    println!("EVCXR_END_CONTENT");
}
