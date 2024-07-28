use super::utils;
use crate::extractors::{config_builder::VMConfig, extractor};
use crate::transformers;
use swc_common::{chain, comments::SingleThreadedComments, Mark, GLOBALS};
use swc_core::ecma::transforms;
use swc_ecma_transforms::optimization::simplifier;
use swc_ecma_transforms::optimization::simplify::{expr_simplifier, Config};
use swc_ecma_transforms::pass::noop;
use swc_ecma_transforms::{fixer, resolver};
use swc_ecma_visit::as_folder;

pub fn deobfuscate(cnfg: &mut VMConfig, src: &str) -> String {
    let mut out: String = "".to_owned();

    GLOBALS.set(&Default::default(), || {
        let marks = Mark::new();
        let data = utils::get_structs(src);
        let output = data
            .comp
            .process_js_with_custom_pass(
                data.source,
                None,
                &data.handler,
                &utils::create_transform_options(),
                SingleThreadedComments::default(),
                |_| noop(),
                |_| {
                    chain!(
                        resolver(marks, marks, false),
                        as_folder(transformers::strings::Visitor::new(src.to_string())),
                        as_folder(transformers::proxy_functions::Visitor),
                        as_folder(transformers::computed_members::Visitor),
                        as_folder(transformers::control_flow_flattening::Visitor),
                        as_folder(transformers::cleanup_deleted::Visitor),
                        as_folder(transformers::sequence_expressions::Visitor),
                        expr_simplifier(marks, Default::default()),
                        transforms::optimization::simplify::dce::dce(Default::default(), marks,),
                        as_folder(transformers::useless_if::Visitor),
                        as_folder(transformers::simplify_binary::Visitor),
                        // extractor: Only required for parsing the script, not deobfuscating it
                        as_folder(extractor::Visitor { cnfg: cnfg }),
                        simplifier(marks, Config::default()),
                        fixer(None),
                    )
                },
            )
            .expect("process_js_with_custom_pass failed");

        out = output.code;
    });
    return out;
}
