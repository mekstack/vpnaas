fn main() {
    tonic_build::compile_protos("../proto/vpnaas.proto")
        .unwrap_or_else(|e| panic!("Could not compile protos {:?}", e));
}
