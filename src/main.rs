use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::fs;
use std::io::{LineWriter, Write as IoWrite};

static NX: i32 = 200;
static NY: i32 = 100;
static MAX_COLOR: i32 = 255;
static OUTPUT_DIR: &str = "output";

fn main() -> std::io::Result<()> {
    fs::create_dir_all(OUTPUT_DIR)?;
    let file = File::create(format!("{}/chapter1.ppm", OUTPUT_DIR))?;
    let mut file = LineWriter::new(file);

    let mut header = String::new();
    write!(header, "P3\n{} {}\n{}\n", NX, NY, MAX_COLOR);
    file.write_all(header.as_bytes())?;

    let mut line = String::new();
    for j in (0..NY).rev() {
        for i in 0..NX {
            let r: f64 = i as f64 / NX as f64;
            let g: f64 = j as f64 / NY as f64;
            let b = 0.2;
            let ir = (MAX_COLOR as f64 * r) as i32;
            let ig = (MAX_COLOR as f64 * g) as i32;
            let ib = (MAX_COLOR as f64 * b) as i32;
            write!(line, "{r} {g} {b}\n", r = ir, g = ig, b = ib);
            file.write_all(line.as_bytes())?;
            line.clear();
        }
    }

    Ok(())
}
