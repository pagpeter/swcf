use std::io;
use swc::config::{Config, IsModule, JscConfig, ModuleConfig, Options};
use swc::Compiler;
use swc_common::{errors::Handler, source_map::SourceMap, sync::Lrc};
use swc_ecma_ast::EsVersion;
use swc_ecma_parser::{EsConfig, Syntax};
use swc_ecma_transforms::modules::EsModuleConfig;

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

pub struct Data {
    pub comp: Compiler,
    pub handler: Handler,
    pub source: Lrc<swc_common::SourceFile>,
}

pub fn get_structs(data: &str) -> Data {
    let cm = Lrc::new(SourceMap::new(swc_common::FilePathMapping::empty()));
    let c = swc::Compiler::new(cm.clone());
    let handler = Handler::with_emitter_writer(Box::new(io::stderr()), Some(c.cm.clone()));
    let source: Lrc<swc_common::SourceFile> =
        cm.new_source_file(swc_common::FileName::Anon, data.to_string());
    return Data {
        comp: c,
        handler,
        source,
    };
}
