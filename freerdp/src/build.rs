extern crate bindgen;

use std::env;
use std::path::PathBuf;
use walkdir::WalkDir;
use std::fs::File;
use std::io::Write;

fn main() {
    let library = pkg_config::probe_library("freerdp3")
        .expect("pkg-config could not find freerdp3. Try setting PKG_CONFIG_PATH if not installed in system directories");

    let ferr = "Failed to write wrapper.h";
    let out_path = PathBuf::from(env::var("OUT_DIR").expect("Environment variable OUT_DIR not set"));

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let mut builder = bindgen::Builder::default();

    let header_file = out_path.join("wrapper.h");
    let mut header = File::create(&header_file).expect(ferr);
    writeln!(&mut header, "").expect(ferr);
    writeln!(&mut header, "#ifndef FREERDP_3_WRAPPER_HEADER").expect(ferr);
    writeln!(&mut header, "#define FREERDP_3_WRAPPER_HEADER").expect(ferr);
    writeln!(&mut header, "").expect(ferr);

    for path in &library.include_paths {
        let dpath = path.display();
        let spath = dpath.to_string();
        builder = builder.clang_args(&["-I", &spath]);

        if !spath.contains("freerdp3") {
            continue;
        }

        for file in WalkDir::new(&path).into_iter().filter_map(|file| file.ok()) {
            if !file.metadata().unwrap().is_file() {
                continue;
            }
            let rpath = file.path().strip_prefix(&path).expect("failed to strip absolute path");
            writeln!(&mut header, "#include <{}>", rpath.display()).expect(ferr);
        }
    }

    writeln!(&mut header, "").expect(ferr);
    writeln!(&mut header, "#endif /* FREERDP_3_WRAPPER_HEADER */").expect(ferr);
    writeln!(&mut header, "").expect(ferr);

    builder = builder.header(&header_file.display().to_string());
    let bindings = builder.parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
