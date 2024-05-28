mod utils;

use std::{env, fs, time};
use swc_common::{chain, Mark};
use swc_common::{comments::SingleThreadedComments, GLOBALS};
use swc_core::ecma::visit::as_folder;
use swc_ecma_transforms::optimization::simplify::expr_simplifier;
use swc_ecma_transforms::pass::noop;

mod transformations;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1);
    if filename.is_none() {
        return println!("You must pass in the file path");
    }

    let src = fs::read_to_string(filename.unwrap()).expect("Unable to read file");
    let before = time::Instant::now();

    println!("[!] Elapsed time (Parsing): {:.2?}", before.elapsed());

    let data = utils::get_structs(&src);

    GLOBALS.set(&Default::default(), || {
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
                    )
                },
            )
            .expect("process_js_with_custom_pass failed");
        fs::write(format!("{}_out.js", filename.unwrap()), output.code)
            .expect("Could not write to file");
        println!(
            "[!] Elapsed time (Parsing + Transforming + Marshalling + Writing): {:.2?}",
            before.elapsed()
        );
    })
}
