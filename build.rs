fn main() {
    tonic_build::configure()
        .type_attribute(
            ".",
            "#[derive(Hash, Eq, serde::Serialize, serde::Deserialize)]",
        )
        .out_dir("src/proto")
        .compile(&["./login.proto"], &["./"])
        .unwrap();
    println!("cargo:rerun-if-changed=./login.proto");
    println!("cargo:rerun-if-changed=./build.rs");
}
