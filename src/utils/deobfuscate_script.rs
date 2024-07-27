use super::utils;
use crate::extractors::{config_builder::VMConfig, extractor};
use crate::transformers;
use swc_common::{chain, comments::SingleThreadedComments, Mark, GLOBALS};
use swc_ecma_transforms::optimization::simplify::expr_simplifier;
use swc_ecma_transforms::pass::noop;
use swc_ecma_visit::as_folder;

pub fn deobfuscate(cnfg: &mut VMConfig, src: &str) -> String {
    let mut out: String = "".to_owned();

    GLOBALS.set(&Default::default(), || {
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
                        as_folder(transformers::strings::Visitor::new(src.to_string())),
                        as_folder(transformers::proxy_functions::Visitor),
                        as_folder(transformers::computed_members::Visitor),
                        as_folder(transformers::control_flow_flattening::Visitor),
                        as_folder(transformers::cleanup_deleted::Visitor),
                        as_folder(transformers::sequence_expressions::Visitor),
                        expr_simplifier(Mark::new(), Default::default()),
                        as_folder(transformers::useless_if::Visitor),
                        as_folder(transformers::simplify_binary::Visitor),
                        as_folder(extractor::Visitor { cnfg: cnfg }),
                    )
                },
            )
            .expect("process_js_with_custom_pass failed");

        out = output.code;
    });
    return out;
}
