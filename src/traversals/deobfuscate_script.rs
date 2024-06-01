use super::utils;
use crate::traversals::{config_builder::VMConfig, extractor, transformations};
use swc_common::{chain, comments::SingleThreadedComments, Mark, GLOBALS};
use swc_ecma_transforms::optimization::simplify::expr_simplifier;
use swc_ecma_transforms::pass::noop;
use swc_ecma_visit::as_folder;

pub fn deobfuscate(cnfg: VMConfig, src: &str) -> (String, VMConfig) {
    let mut out: String = "".to_owned();
    let mut out_config: VMConfig = Default::default();

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
                        as_folder(transformations::strings::Visitor::new(src.to_string())),
                        as_folder(transformations::proxy_functions::Visitor),
                        as_folder(transformations::computed_members::Visitor),
                        as_folder(transformations::control_flow_flattening::Visitor),
                        as_folder(transformations::cleanup_deleted::Visitor),
                        as_folder(transformations::sequence_expressions::Visitor),
                        expr_simplifier(Mark::new(), Default::default()),
                        as_folder(transformations::useless_if::Visitor),
                        as_folder(transformations::simplify_binary::Visitor),
                        as_folder(extractor::Visitor),
                    )
                },
            )
            .expect("process_js_with_custom_pass failed");

        out = output.code;
        out_config = cnfg.to_owned();
    });
    return (out, out_config);
}
