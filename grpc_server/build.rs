fn main() {
    // let out_dir = std::env::var("OUT_DIR").unwrap();
    // println!("{out_dir}");
    tonic_build::compile_protos("proto/route_guide.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
    // tonic_build::compile_protos("proto/diary.proto")
    //     .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
