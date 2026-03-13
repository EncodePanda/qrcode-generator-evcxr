# qrcode-generator-evcxr Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a thin Rust library that wraps `qrcode-generator` to display QR codes as SVG in Jupyter notebooks via the evcxr kernel.

**Architecture:** A single-file library (`src/lib.rs`) exposing two public functions (`draw_qrcode`, `draw_qrcode_with_config`), a `QrCodeable` trait with implementations for string types and `Vec<Vec<bool>>`, a `QrConfig` struct, and an internal SVG renderer. The `qrcode-generator` crate is used only for matrix generation; SVG is produced by our own renderer.

**Tech Stack:** Rust, `qrcode-generator` 5.0.0, evcxr Jupyter kernel

**Spec:** `docs/superpowers/specs/2026-03-13-qrcode-generator-evcxr-design.md`

---

## File Structure

- **Create:** `Cargo.toml` — crate metadata and `qrcode-generator` dependency
- **Create:** `src/lib.rs` — all public API, trait, config, SVG renderer, and evcxr output

---

## Chunk 1: Project Scaffold and SVG Renderer

### Task 1: Initialize the Rust project

**Files:**
- Create: `Cargo.toml`
- Create: `src/lib.rs`

- [ ] **Step 1: Create `Cargo.toml`**

```toml
[package]
name = "qrcode-generator-evcxr"
version = "0.1.0"
edition = "2021"
description = "Display QR codes in Jupyter notebooks via evcxr"
license = "MIT"

[dependencies]
qrcode-generator = "5.0.0"
```

- [ ] **Step 2: Create `src/lib.rs` with re-export and config**

```rust
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
```

- [ ] **Step 3: Verify it compiles**

Run: `cargo build`
Expected: compiles successfully with no errors

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml src/lib.rs
git commit -m "feat: scaffold project with QrConfig and QrCodeEcc re-export"
```

---

### Task 2: Implement the SVG renderer

**Files:**
- Modify: `src/lib.rs`
- Create: `tests/svg_renderer.rs`

- [ ] **Step 1: Write failing test for SVG renderer**

Create `tests/svg_renderer.rs`:

```rust
use qrcode_generator_evcxr::render_svg;

#[test]
fn test_render_svg_produces_valid_svg() {
    // A tiny 2x2 matrix: top-left and bottom-right are black
    let matrix = vec![
        vec![true, false],
        vec![false, true],
    ];
    let svg = render_svg(&matrix, 100, 0);

    assert!(svg.starts_with("<svg"));
    assert!(svg.contains("width=\"100\""));
    assert!(svg.contains("height=\"100\""));
    // With 2x2 matrix and no margin, each cell is 50x50
    // Cell (0,0) is true -> black rect at (0,0) size 50x50
    assert!(svg.contains("x=\"0\" y=\"0\" width=\"50\" height=\"50\""));
    // Cell (1,1) is true -> black rect at (50,50) size 50x50
    assert!(svg.contains("x=\"50\" y=\"50\" width=\"50\" height=\"50\""));
    assert!(svg.ends_with("</svg>"));
}

#[test]
fn test_render_svg_with_margin() {
    // 1x1 matrix with margin of 1 -> total grid is 3x3 (1 margin + 1 data + 1 margin)
    let matrix = vec![vec![true]];
    let svg = render_svg(&matrix, 300, 1);

    assert!(svg.contains("width=\"300\""));
    assert!(svg.contains("height=\"300\""));
    // 3x3 grid in 300px -> each cell is 100x100
    // The data cell is at grid position (1,1) -> pixel (100,100)
    assert!(svg.contains("x=\"100\" y=\"100\" width=\"100\" height=\"100\""));
}

#[test]
fn test_render_svg_empty_matrix() {
    let matrix: Vec<Vec<bool>> = vec![];
    let svg = render_svg(&matrix, 100, 0);
    assert!(svg.starts_with("<svg"));
    assert!(svg.ends_with("</svg>"));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test svg_renderer`
Expected: FAIL — `render_svg` is not public / doesn't exist yet

- [ ] **Step 3: Implement `render_svg` in `src/lib.rs`**

Add to `src/lib.rs`:

```rust
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
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --test svg_renderer`
Expected: all 3 tests PASS

- [ ] **Step 5: Commit**

```bash
git add src/lib.rs tests/svg_renderer.rs
git commit -m "feat: implement SVG renderer with margin support"
```

---

## Chunk 2: QrCodeable Trait and Public API

### Task 3: Implement the QrCodeable trait

**Files:**
- Modify: `src/lib.rs`
- Create: `tests/qrcodeable.rs`

- [ ] **Step 1: Write failing tests for QrCodeable**

Create `tests/qrcodeable.rs`:

```rust
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
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test qrcodeable`
Expected: FAIL — `QrCodeable` trait doesn't exist yet

- [ ] **Step 3: Implement `QrCodeable` trait in `src/lib.rs`**

Add to `src/lib.rs`:

```rust
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
```

**Note:** A blanket `impl<T: AsRef<str>>` would be cleaner but conflicts with the `Vec<Vec<bool>>` impl because Rust cannot guarantee `Vec<Vec<bool>>` will never implement `AsRef<str>`. The explicit impls for `str`, `String`, and the `&T` forwarding impl cover all common usage: `&str`, `String`, `&String`, `&&str`, etc.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --test qrcodeable`
Expected: all 4 tests PASS

- [ ] **Step 5: Commit**

```bash
git add src/lib.rs tests/qrcodeable.rs
git commit -m "feat: implement QrCodeable trait for string types and Vec<Vec<bool>>"
```

---

### Task 4: Implement public draw functions

**Files:**
- Modify: `src/lib.rs`
- Create: `tests/draw.rs`

- [ ] **Step 1: Write failing tests for draw functions**

Create `tests/draw.rs`:

```rust
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
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test draw`
Expected: FAIL — `draw_qrcode` and `draw_qrcode_with_config` don't exist yet

- [ ] **Step 3: Implement draw functions in `src/lib.rs`**

Add to `src/lib.rs`:

```rust
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
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --test draw`
Expected: all 5 tests PASS

- [ ] **Step 5: Run full test suite**

Run: `cargo test`
Expected: all tests PASS across all test files

- [ ] **Step 6: Commit**

```bash
git add src/lib.rs tests/draw.rs
git commit -m "feat: implement draw_qrcode and draw_qrcode_with_config"
```

---

## Chunk 3: Final Verification

### Task 5: Final review and cleanup

**Files:**
- Review: `src/lib.rs`

- [ ] **Step 1: Verify public API exports**

Run: `cargo doc --no-deps`
Expected: docs build successfully. Verify that these items are public:
- `draw_qrcode`
- `draw_qrcode_with_config`
- `QrConfig`
- `QrCodeable`
- `QrCodeEcc` (re-export)
- `render_svg`

- [ ] **Step 2: Run full test suite one final time**

Run: `cargo test`
Expected: all tests PASS

- [ ] **Step 3: Run clippy**

Run: `cargo clippy -- -D warnings`
Expected: no warnings

- [ ] **Step 4: Commit any cleanup**

Only if clippy or doc review surfaced changes:

```bash
git add -A
git commit -m "chore: address clippy warnings and cleanup"
```
