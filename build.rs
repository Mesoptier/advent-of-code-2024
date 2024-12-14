use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::{env, fs};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/days");

    // Collect days `(mod name, mod path)`
    let mut days = vec![];
    for entry in fs::read_dir("src/days").unwrap() {
        let entry = entry.unwrap();
        let day_name = entry
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let day_path = entry
            .path()
            .to_str()
            .unwrap()
            .to_string()
            .replace('\\', "\\\\");

        // Skip day template
        if day_name == "dayN" {
            continue;
        }

        days.push((day_name, day_path));
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_days_path = Path::new(&out_dir).join("days.in");
    let out_days_file = File::create(out_days_path).unwrap();
    let mut out_days_writer = BufWriter::new(out_days_file);

    // Attach mods, with absolute path specified to the actual mod file
    for (day_name, day_path) in &days {
        writeln!(out_days_writer, "#[path = \"{}\"]", day_path).unwrap();
        writeln!(out_days_writer, "pub mod {};", day_name).unwrap();
    }

    // Implement wrapper functions
    writeln!(out_days_writer, "impl_days! {{").unwrap();
    for (day_name, _) in &days {
        writeln!(out_days_writer, "    {},", day_name).unwrap();
    }
    writeln!(out_days_writer, "}}").unwrap();

    out_days_writer.flush().unwrap();
}
