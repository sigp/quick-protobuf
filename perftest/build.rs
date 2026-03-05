extern crate pb_rs;
extern crate prost_build;
extern crate protobuf_codegen;

use pb_rs::types::{Config, FileDescriptor, RpcService};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};

fn generate_rpc_test<W: Write + ?Sized>(
    rpc: &RpcService,
    w: &mut W,
) -> Result<(), pb_rs::errors::Error> {
    /* Example:
        trait <service> {
            fn <func>(&self, arg: &<arg>) -> Result<<ret>, failure::Error>;
        }
    */

    writeln!(w, "\npub trait {SERVICE} {{", SERVICE = rpc.service_name)?;
    for func in rpc.functions.iter() {
        writeln!(
            w,
            "   fn {FUNC}(&self, arg: &{ARG}) -> std::result::Result<{RET}, quick_protobuf::Error>;",
            FUNC = func.name,
            ARG = func.arg,
            RET = func.ret
        )?;
    }
    writeln!(w, "}}\n")?;

    Ok(())
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let rust_pb_out_dir = Path::new(&manifest_dir).join("src/generated_rust");
    fs::create_dir_all(&rust_pb_out_dir).unwrap();

    // protobuf
    protobuf_codegen::Codegen::new()
        .pure()
        .out_dir(&rust_pb_out_dir)
        .inputs(["src/perftest_data.proto"])
        .include("src")
        .run()
        .expect("protoc");

    // quick-protobuf
    let quick_dest = Path::new(&out_dir).join("perftest_data_quick.rs");
    let config = Config {
        in_file: PathBuf::from("src/perftest_data.proto"),
        out_file: quick_dest,
        single_module: true,
        import_search_path: vec![PathBuf::from("src")],
        no_output: false,
        error_cycle: false,
        headers: false,
        dont_use_cow: false,
        custom_struct_derive: vec![],
        custom_repr: None,
        custom_rpc_generator: Box::new(|rpc, writer| generate_rpc_test(rpc, writer)),
        custom_includes: Vec::new(),
        owned: false,
        hashbrown: false,
        nostd: false,
        gen_info: false,
        add_deprecated_fields: false,
        generate_getters: true,
    };
    FileDescriptor::write_proto(&config).unwrap();

    // prost - skip if protoc is not available
    let old = ::std::env::var("OUT_DIR");
    env::set_var("OUT_DIR", ".");
    match prost_build::compile_protos(&["src/perftest_data.proto"], &["src"]) {
        Ok(_) => {
            let _ = old.map(|val| env::set_var("OUT_DIR", val));
            fs::rename("perftest_data.rs", "src/perftest_data_prost.rs").unwrap();
        }
        Err(e) => {
            let _ = old.map(|val| env::set_var("OUT_DIR", val));
            println!("cargo:warning=prost codegen skipped: {}", e);
            println!("cargo:warning=Install protoc to enable prost benchmarks");
        }
    }
}
