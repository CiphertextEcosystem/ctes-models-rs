fn main() {
    let mut config = prost_build::Config::new();
    config.out_dir("src/generated");
    config.compile_protos(&["proto/model/ciphertext.proto"][..], &["proto/"][..]).unwrap();
}