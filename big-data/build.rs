fn main() {
    protobuf_codegen::Codegen::new()
        .includes(&["src"])
        .input("src/big-data.proto")
        .cargo_out_dir("protos")
        .run_from_script();
}
