use std::io;
use std::{env, fs, time};
use swc::config::Options;
// use swc_common::Mark;
use swc_common::{
    comments::SingleThreadedComments, errors::Handler, source_map::SourceMap, sync::Lrc, GLOBALS,
};
use swc_core::ecma::visit::as_folder;
use swc_ecma_transforms::pass::noop;
mod transformations;
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
    println!("Elapsed time (Parsing): {:.2?}", before.elapsed());

    let globals = Default::default();
    return GLOBALS.set(&globals, || {
        let output = c
            .process_js_with_custom_pass(
                source,
                None,
                &handler,
                &Options::default(),
                SingleThreadedComments::default(),
                |_| noop(),
                |_| {
                    swc_common::chain!(
                        // expr_simplifier(Mark::new(), Default::default()),
                        // as_folder(transformations::proxy_vars::Visitor::default()),
                        as_folder(transformations::strings::Visitor::new(data.to_string())),
                        as_folder(transformations::computed_members::Visitor)
                    )
                },
            )
            .expect("process_js_with_custom_pass failed");
        fs::write(format!("{}_out.js", filename), output.code).expect("Could not write to file");
        println!(
            "Elapsed time (Parsing + Transforming + Marshalling + Writing): {:.2?}",
            before.elapsed()
        );
    });
}
