use std::io;
use std::{env, fs, time};
use swc::config::{Config, IsModule, JscConfig, ModuleConfig, Options};
use swc_common::{chain, Mark};
use swc_common::{
    comments::SingleThreadedComments, errors::Handler, source_map::SourceMap, sync::Lrc, GLOBALS,
};
use swc_core::ecma::visit::as_folder;
use swc_ecma_ast::EsVersion;
use swc_ecma_parser::{EsConfig, Syntax};
use swc_ecma_transforms::modules::EsModuleConfig;
use swc_ecma_transforms::optimization::simplify::expr_simplifier;
use swc_ecma_transforms::pass::noop;

mod transformations;

pub fn create_transform_options() -> Options {
    let jsc = JscConfig {
        external_helpers: false.into(),
        keep_class_names: true.into(),
        loose: true.into(),
        minify: None,
        preserve_all_comments: true.into(),
        syntax: Some(Syntax::Es(EsConfig {
            allow_super_outside_method: false,
            allow_return_outside_function: false,
            decorators_before_export: true,
            export_default_from: true,
            fn_bind: true,
            jsx: true,
            ..EsConfig::default()
        })),

        target: Some(EsVersion::latest()),
        ..JscConfig::default()
    };
    Options {
        config: Config {
            // env,
            is_module: Some(IsModule::Bool(true)),
            jsc,
            minify: false.into(),
            module: Some(ModuleConfig::Es6(EsModuleConfig {
                resolve_fully: false,
            })),
            ..Config::default()
        },
        ..Options::default()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = match args.get(1) {
        Some(v) => v,
        None => {
            println!("You must pass in the file path");
            return;
        }
    };

    let data = fs::read_to_string(filename).expect("Unable to read file");

    let before = time::Instant::now();
    let cm = Lrc::new(SourceMap::new(swc_common::FilePathMapping::empty()));
    let c = swc::Compiler::new(cm.clone());
    let handler = Handler::with_emitter_writer(Box::new(io::stderr()), Some(c.cm.clone()));
    let source = cm.new_source_file(
        swc_common::FileName::Custom(filename.into()),
        data.to_string(),
    );
    println!("[!] Elapsed time (Parsing): {:.2?}", before.elapsed());

    let globals = Default::default();
    GLOBALS.set(&globals, || {
        let output = c
            .process_js_with_custom_pass(
                source,
                None,
                &handler,
                &create_transform_options(),
                SingleThreadedComments::default(),
                |_| noop(),
                |_| {
                    chain!(
                        as_folder(transformations::strings::Visitor::new(data.to_string())),
                        as_folder(transformations::proxy_functions::Visitor),
                        as_folder(transformations::computed_members::Visitor),
                        as_folder(transformations::control_flow_flattening::Visitor),
                        as_folder(transformations::cleanup_deleted::Visitor),
                        as_folder(transformations::sequence_expressions::Visitor),
                        expr_simplifier(Mark::new(), Default::default()),
                        as_folder(transformations::useless_if::Visitor),
                        as_folder(transformations::simplify_binary::Visitor),
                    )
                },
            )
            .expect("process_js_with_custom_pass failed");
        fs::write(format!("{}_out.js", filename), output.code).expect("Could not write to file");
        println!(
            "[!] Elapsed time (Parsing + Transforming + Marshalling + Writing): {:.2?}",
            before.elapsed()
        );
    })
}
