//! `lumen-icon-gen` — PoC for ADR-0008.
//!
//! Converts SVG path data into `lumen-ui-icons`-style painter code at build time,
//! so the published `lumen-ui-icons` crate carries **no** SVG/runtime dependency:
//! only this tool depends on `svgtypes`. The pipeline is
//! `SVG path d="…"` → absolute segments (`svgtypes::SimplifyingPathParser`) →
//! flattened polylines in normalized `0..1` coordinates → generated Rust.
//!
//! Usage:
//! ```text
//! lumen-icon-gen --demo                 # print generated code for built-in samples
//! lumen-icon-gen <in_dir> <out_file>    # generate one module from a dir of *.svg
//! ```
//!
//! Scope of the PoC: handles `<path d>` data (the simplifier resolves rel/smooth/
//! H/V/arc to absolute Move/Line/Curve/Quadratic/Close). The production tool will
//! also resolve `<circle>/<rect>/<line>` and transforms via `usvg` (ADR-0008 §6.3).

use std::process::ExitCode;

use svgtypes::{SimplePathSegment, SimplifyingPathParser};

/// Subdivision steps when flattening one Bézier segment into line points.
const FLATTEN_STEPS: usize = 16;
/// Lucide icons are authored on a 24×24 grid.
const LUCIDE_VIEWBOX: f32 = 24.0;

/// A polyline in normalized (`0..=1`) coordinates.
type Polyline = Vec<(f32, f32)>;

/// Representative SVG path data for the PoC. `check` and `chevron-down` are the
/// exact Lucide (ISC) paths; `curve-demo` exercises cubic + smooth-cubic flattening.
const SAMPLES: &[(&str, &str)] = &[
    ("check", "M20 6 9 17l-5-5"),
    ("chevron_down", "m6 9 6 6 6-6"),
    ("curve_demo", "M4 12 C4 8 8 4 12 4 S20 8 20 12"),
];

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [flag] if flag == "--demo" => {
            print!("{}", generate_module(SAMPLES));
            ExitCode::SUCCESS
        }
        [in_dir, out_file] => match run(in_dir, out_file) {
            Ok(count) => {
                eprintln!("generated {count} icon(s) → {out_file}");
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("error: {e}");
                ExitCode::FAILURE
            }
        },
        _ => {
            eprintln!("usage: lumen-icon-gen --demo | <in_dir> <out_file>");
            ExitCode::FAILURE
        }
    }
}

/// Read every `*.svg` in `in_dir`, generate a module, and write it to `out_file`.
fn run(in_dir: &str, out_file: &str) -> Result<usize, String> {
    let mut icons: Vec<(String, String)> = Vec::new();
    let entries = std::fs::read_dir(in_dir).map_err(|e| format!("read_dir {in_dir}: {e}"))?;
    for entry in entries {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.extension().and_then(|s| s.to_str()) != Some("svg") {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("invalid file name")?
            .replace('-', "_");
        let svg = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let d = extract_path_data(&svg);
        if d.is_empty() {
            eprintln!("warning: no <path d> found in {}", path.display());
            continue;
        }
        icons.push((stem, d));
    }
    icons.sort();
    let samples: Vec<(&str, &str)> = icons
        .iter()
        .map(|(n, d)| (n.as_str(), d.as_str()))
        .collect();
    std::fs::write(out_file, generate_module(&samples)).map_err(|e| e.to_string())?;
    Ok(samples.len())
}

/// Extract and concatenate all `d="…"` attribute values from raw SVG text.
/// Minimal by design (PoC); production uses `usvg` for full correctness.
fn extract_path_data(svg: &str) -> String {
    let mut out = String::new();
    let mut rest = svg;
    while let Some(start) = rest.find("d=\"") {
        let after = &rest[start + 3..];
        if let Some(end) = after.find('"') {
            if !out.is_empty() {
                out.push(' ');
            }
            out.push_str(&after[..end]);
            rest = &after[end + 1..];
        } else {
            break;
        }
    }
    out
}

/// Parse SVG path data into flattened polylines in normalized `0..=1` coordinates.
fn svg_path_to_polylines(d: &str, viewbox: f32) -> Result<Vec<Polyline>, String> {
    let mut polylines: Vec<Polyline> = Vec::new();
    let mut current: Polyline = Vec::new();
    let (mut cx, mut cy) = (0.0_f64, 0.0_f64);
    let (mut sx, mut sy) = (0.0_f64, 0.0_f64); // subpath start, for ClosePath
    let norm = |x: f64, y: f64| ((x as f32) / viewbox, (y as f32) / viewbox);

    for seg in SimplifyingPathParser::from(d) {
        match seg.map_err(|e| format!("{e:?}"))? {
            SimplePathSegment::MoveTo { x, y } => {
                flush(&mut current, &mut polylines);
                cx = x;
                cy = y;
                sx = x;
                sy = y;
                current.push(norm(x, y));
            }
            SimplePathSegment::LineTo { x, y } => {
                cx = x;
                cy = y;
                current.push(norm(x, y));
            }
            SimplePathSegment::CurveTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => {
                flatten_cubic((cx, cy), (x1, y1), (x2, y2), (x, y), &norm, &mut current);
                cx = x;
                cy = y;
            }
            SimplePathSegment::Quadratic { x1, y1, x, y } => {
                flatten_quadratic((cx, cy), (x1, y1), (x, y), &norm, &mut current);
                cx = x;
                cy = y;
            }
            SimplePathSegment::ClosePath => {
                current.push(norm(sx, sy));
                cx = sx;
                cy = sy;
                flush(&mut current, &mut polylines);
            }
        }
    }
    flush(&mut current, &mut polylines);
    Ok(polylines)
}

/// Move `current` into `polylines` if it forms a drawable line (≥ 2 points).
fn flush(current: &mut Polyline, polylines: &mut Vec<Polyline>) {
    if current.len() >= 2 {
        polylines.push(std::mem::take(current));
    } else {
        current.clear();
    }
}

fn flatten_cubic(
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
    norm: &impl Fn(f64, f64) -> (f32, f32),
    out: &mut Polyline,
) {
    for i in 1..=FLATTEN_STEPS {
        let t = i as f64 / FLATTEN_STEPS as f64;
        let mt = 1.0 - t;
        let a = mt * mt * mt;
        let b = 3.0 * mt * mt * t;
        let c = 3.0 * mt * t * t;
        let d = t * t * t;
        let x = a * p0.0 + b * p1.0 + c * p2.0 + d * p3.0;
        let y = a * p0.1 + b * p1.1 + c * p2.1 + d * p3.1;
        out.push(norm(x, y));
    }
}

fn flatten_quadratic(
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
    norm: &impl Fn(f64, f64) -> (f32, f32),
    out: &mut Polyline,
) {
    for i in 1..=FLATTEN_STEPS {
        let t = i as f64 / FLATTEN_STEPS as f64;
        let mt = 1.0 - t;
        let a = mt * mt;
        let b = 2.0 * mt * t;
        let c = t * t;
        let x = a * p0.0 + b * p1.0 + c * p2.0;
        let y = a * p0.1 + b * p1.1 + c * p2.1;
        out.push(norm(x, y));
    }
}

/// Emit a `paint_<name>` function drawing the polylines, matching the painter style
/// of `lumen-ui-icons` (normalized coords mapped through an `at(rect, x, y)` helper).
fn emit_icon_fn(name: &str, polylines: &[Polyline]) -> String {
    let mut s = format!(
        "fn paint_{name}(painter: &egui::Painter, rect: egui::Rect, stroke: egui::Stroke) {{\n"
    );
    for line in polylines {
        s.push_str("    painter.add(egui::Shape::line(vec![\n");
        for (x, y) in line {
            s.push_str(&format!("        at(rect, {x:.4}, {y:.4}),\n"));
        }
        s.push_str("    ], stroke));\n");
    }
    s.push_str("}\n");
    s
}

/// Generate a complete module from `(name, path_data)` pairs.
fn generate_module(samples: &[(&str, &str)]) -> String {
    let mut out = String::from(
        "// @generated by lumen-icon-gen (ADR-0008 PoC). Do not edit by hand.\n\
         // Icons derived from Lucide (ISC License). Coordinates are normalized 0..1.\n\n",
    );
    for (name, d) in samples {
        match svg_path_to_polylines(d, LUCIDE_VIEWBOX) {
            Ok(polylines) => out.push_str(&emit_icon_fn(name, &polylines)),
            Err(e) => out.push_str(&format!("// {name}: parse error: {e}\n")),
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icon_is_one_polyline_of_three_points() {
        // Lucide "check": M20 6 9 17 l-5-5 → MoveTo + LineTo + relative LineTo.
        let pl = svg_path_to_polylines("M20 6 9 17l-5-5", 24.0).unwrap();
        assert_eq!(pl.len(), 1, "one continuous stroke");
        assert_eq!(pl[0].len(), 3, "three vertices");
        assert!(
            (pl[0][0].0 - 20.0 / 24.0).abs() < 1e-5,
            "start x normalized"
        );
        assert!(
            (pl[0][2].0 - 4.0 / 24.0).abs() < 1e-5,
            "end x = 9-5 = 4, normalized"
        );
    }

    #[test]
    fn curve_is_flattened_into_many_points() {
        let pl = svg_path_to_polylines("M4 12 C4 8 8 4 12 4 S20 8 20 12", 24.0).unwrap();
        assert_eq!(pl.len(), 1);
        // 1 MoveTo + 2 curve segments × FLATTEN_STEPS points.
        assert_eq!(pl[0].len(), 1 + 2 * FLATTEN_STEPS);
    }

    #[test]
    fn closepath_returns_to_subpath_start() {
        let pl = svg_path_to_polylines("M2 2 L20 2 L20 20 Z", 24.0).unwrap();
        assert_eq!(pl.len(), 1);
        let first = pl[0][0];
        let last = *pl[0].last().unwrap();
        assert!((first.0 - last.0).abs() < 1e-6 && (first.1 - last.1).abs() < 1e-6);
    }

    #[test]
    fn normalization_keeps_coords_in_unit_range() {
        for (_, d) in SAMPLES {
            let pls = svg_path_to_polylines(d, 24.0).unwrap();
            for pl in pls {
                for (x, y) in pl {
                    assert!((0.0..=1.0).contains(&x), "x in 0..1");
                    assert!((0.0..=1.0).contains(&y), "y in 0..1");
                }
            }
        }
    }

    #[test]
    fn codegen_emits_drawable_function() {
        let pl = svg_path_to_polylines("M20 6 9 17l-5-5", 24.0).unwrap();
        let code = emit_icon_fn("check", &pl);
        assert!(code.contains("fn paint_check"));
        assert!(code.contains("egui::Shape::line"));
        assert!(code.contains("at(rect,"));
    }

    #[test]
    fn extract_path_data_concatenates_multiple_paths() {
        let svg = r#"<svg><path d="M5 12h14"/><path d="M12 5v14"/></svg>"#;
        assert_eq!(extract_path_data(svg), "M5 12h14 M12 5v14");
    }
}
