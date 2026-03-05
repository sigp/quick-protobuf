use clap::{Arg, ArgAction, Command};
use pb_rs::{errors::Error, types::FileDescriptor, ConfigBuilder};
use std::path::{Path, PathBuf};

fn run() -> Result<(), Error> {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("OUTPUT")
                .required(false)
                .long("output")
                .short('o')
                .value_parser(|x: &str| extension_matches(x, "rs"))
                .help("Generated file name, defaults to INPUT with 'rs' extension, cannot be used with --output_directory"),
        ).arg(
            Arg::new("OUTPUT_DIR")
                .required(false)
                .long("output_directory")
                .short('d')
                .help("Output directory of generated code, cannot be used with --output"),
        ).arg(
            Arg::new("INCLUDE_PATH")
                .required(false)
                .long("include")
                .short('I')
                .action(ArgAction::Append)
                .help("Path to search for imported protobufs"),
        ).arg(
            Arg::new("SINGLE_MOD")
                .required(false)
                .long("single-mod")
                .short('s')
                .action(ArgAction::SetTrue)
                .help("Omit generation of modules for each package when there is only one package"),
        ).arg(
            Arg::new("NO_OUTPUT")
                .required(false)
                .long("no-output")
                .short('n')
                .action(ArgAction::SetTrue)
                .help(
                    "Show enums and messages in this .proto file, including those imported. \
                     No code generated",
                ),
        ).arg(
            Arg::new("INPUT")
                .action(ArgAction::Append)
                .required(true)
                .value_parser(|x: &str| extension_matches(x, "proto"))
                .help("The .proto files used to generate quick-protobuf code"),
        ).arg(
            Arg::new("CYCLE")
                .long("error-cycle")
                .short('e')
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Error out if recursive messages do not have optional fields"),
        ).arg(
            Arg::new("NO_HEADERS")
                .long("no-headers")
                .short('H')
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Do not add module comments and module attributes in generated file"),
        ).arg(
            Arg::new("CUSTOM_STRUCT_DERIVE")
                .long("custom_struct_derive")
                .short('C')
                .required(false)
                .help("The comma separated values to add to #[derive(...)] for every struct"),
        ).arg(
            Arg::new("CUSTOM_REPR")
                .long("custom_repr")
                .short('R')
                .required(false)
                .help("The value to use for the optional #[repr(...)] for every struct"),
        ).arg(
            Arg::new("DONT_USE_COW")
                .required(false)
                .long("dont_use_cow")
                .short('D')
                .action(ArgAction::SetTrue)
                .help("Don't use Cow for String and Byte types"),
        ).arg(
            Arg::new("OWNED")
                .long("owned")
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Generate Owned structs when the proto stuct has a lifetime"),
        ).arg(
            Arg::new("NOSTD")
                .long("nostd")
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Generate no_std compliant code"),
        ).arg(
            Arg::new("HASHBROWN")
                .long("hashbrown")
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Use the hashbrown crate as the HashMap implementation"),
        ).arg(
            Arg::new("GEN_INFO")
                .long("gen-info")
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Generate MessageInfo implementations")
        ).arg(
            Arg::new("ADD_DEPRECATED_FIELDS")
                .long("add-deprecated-fields")
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Add deprecated fields and mark them as #[deprecated]")
        ).arg(
            Arg::new("GENERATE_GETTERS")
                .long("generate-getters")
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Generate getters for fields with custom default values.")
        ).get_matches();

    let in_files: Vec<PathBuf> = matches
        .get_many::<String>("INPUT")
        .unwrap_or_default()
        .map(PathBuf::from)
        .collect();
    let include_paths: Vec<PathBuf> = matches
        .get_many::<String>("INCLUDE_PATH")
        .unwrap_or_default()
        .map(PathBuf::from)
        .collect();
    let out_file = matches.get_one::<String>("OUTPUT").map(PathBuf::from);
    let out_dir = matches.get_one::<String>("OUTPUT_DIR").map(PathBuf::from);
    let custom_repr = matches.get_one::<String>("CUSTOM_REPR").map(|o| o.into());
    let custom_struct_derive: Vec<String> = matches
        .get_one::<String>("CUSTOM_STRUCT_DERIVE")
        .map(|s| s.as_str())
        .unwrap_or("")
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    let compiler = ConfigBuilder::new(
        &in_files,
        out_file.as_ref(),
        out_dir.as_ref(),
        &include_paths,
    )?
    .single_module(matches.get_flag("SINGLE_MOD"))
    .no_output(matches.get_flag("NO_OUTPUT"))
    .error_cycle(matches.get_flag("CYCLE"))
    .headers(!matches.get_flag("NO_HEADERS"))
    .dont_use_cow(matches.get_flag("DONT_USE_COW"))
    .custom_struct_derive(custom_struct_derive)
    .nostd(matches.get_flag("NOSTD"))
    .hashbrown(matches.get_flag("HASHBROWN"))
    .gen_info(matches.get_flag("GEN_INFO"))
    .custom_repr(custom_repr)
    .owned(matches.get_flag("OWNED"))
    .add_deprecated_fields(matches.get_flag("ADD_DEPRECATED_FIELDS"))
    .generate_getters(matches.get_flag("GENERATE_GETTERS"));

    FileDescriptor::run(&compiler.build())
}

fn extension_matches<P: AsRef<Path>>(
    path: P,
    expected: &str,
) -> std::result::Result<String, String> {
    match path.as_ref().extension() {
        Some(x) if x == expected => Ok(path.as_ref().to_string_lossy().into_owned()),
        Some(x) => Err(format!(
            "Expected path with extension '{}', not: '{}'",
            expected,
            x.to_string_lossy()
        )),
        None => Err(format!("Expected path with extension '{}'", expected)),
    }
}

fn main() {
    env_logger::init();
    ::std::process::exit({
        if let Err(e) = run() {
            eprintln!("pb-rs fatal error {}", e);
            let mut e: &dyn std::error::Error = &e;
            while let Some(err) = e.source() {
                eprintln!("  - {}", err);
                e = err;
            }
            1
        } else {
            0
        }
    });
}
