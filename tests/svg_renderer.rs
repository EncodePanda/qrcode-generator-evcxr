use qrcode_generator_evcxr::render_svg;

#[test]
fn test_render_svg_produces_valid_svg() {
    // A tiny 2x2 matrix: top-left and bottom-right are black
    let matrix = vec![vec![true, false], vec![false, true]];
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
