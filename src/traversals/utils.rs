use crate::traversals::config_builder::{PayloadKey, VMConfig};
use rand::Rng;
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

// Marshals the init payload - dynamic keys from the script
pub fn get_init_data(init_keys: &Vec<PayloadKey>, cnfg: &VMConfig) -> String {
    let mut j: String = "{".to_owned();
    for k in init_keys {
        if k.value_type == "NUMBER" {
            j += &format!("\"{}\":{},", k.key, k.num_value.round())
        } else if k.value_type == "RANDOM" {
            j += &format!("\"{}\":{},", k.key, rand::thread_rng().gen_range(1..20))
        } else if k.value_type == "SENSOR" {
            j += &format!("\"{}\":{{", k.key);
            for sub in &k.sub_keys {
                j += &format!("\"{}\":0,", sub)
            }
            j += &format!("}},");
        } else if k.value_type == "DATA" {
            let val: String;

            if k.data_key == "cType" {
                val = format!("\"{}\"", cnfg.chl_data.c_type.to_string());
            } else if k.data_key == "cNounce" {
                val = format!("\"{}\"", cnfg.chl_data.c_nounce.to_string());
            } else if k.data_key == "cvId" {
                val = format!("\"{}\"", cnfg.chl_data.cv_id.to_string());
            } else if k.data_key == "cRq" {
                val = serde_json::to_string(&cnfg.chl_data.c_rq).unwrap();
            } else {
                // println!("Not implemented: {}", k.data_key);
                val = "false".to_owned();
            }
            j += &format!("\"{}\":{},", k.key, val);
        }
    }
    j += "}";
    j = j.replace(",}", "}");
    return j;
}
