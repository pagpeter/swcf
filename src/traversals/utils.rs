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

pub fn decrypt_response(input: &str, c_ray: &str) -> String {
    let mut j: i32 = 32;
    let l: String = format!("{}_0", c_ray);

    l.chars().for_each(|s| j ^= s as i32);

    let input_d: Vec<u8> =
        base64::Engine::decode(&base64::prelude::BASE64_STANDARD, input).unwrap();

    let mut out: Vec<String> = vec![];

    let mut i: i32 = 0;
    loop {
        if input_d.len() <= i.try_into().unwrap() {
            break;
        }
        let m: i32 = input_d[i as usize] as i32;
        let m2 = ((m & 255) - j - (i % 65535) + 65535) % 255;
        let char = std::str::from_utf8(&vec![m2 as u8]).unwrap().to_owned();
        out.push(char);

        i += 1;
    }

    return out.join("");
}
