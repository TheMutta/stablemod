fn main() {
    println!("cargo:rerun-if-changed=../abi.h");

    let bindings = bindgen::Builder::default()
        .header("../abi.h")
        .use_core()
        .generate()
        .expect("unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("abi.rs"))
        .expect("couldn't write bindings");


    println!("cargo:rustc-link-search=native=../buildrump.sh/rump_output/lib/librump");
    println!("cargo:rustc-link-lib=static=rump");
    println!("cargo:rustc-link-arg=-z");
    println!("cargo:rustc-link-arg=nostart-stop-gc");
}
