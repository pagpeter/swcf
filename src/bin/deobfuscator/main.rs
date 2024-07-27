use std::{env, fs, time};
use swccf::traversals::{self, config_builder::VMConfig};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1);
    if filename.is_none() {
        return println!("You must pass in the file path");
    }

    let src = fs::read_to_string(filename.unwrap()).expect("Unable to read file");
    let before = time::Instant::now();

    println!("[!] Elapsed time (Parsing): {:.2?}", before.elapsed());

    let mut cnfg = VMConfig::default();
    let out = traversals::deobfuscate_script::deobfuscate(&mut cnfg, &src);

    cnfg.find_all_enc(&src);

    fs::write(format!("{}_out.js", filename.unwrap()), out).expect("Could not write to file");
    println!(
        "[!] Elapsed time (Parsing + Transforming + Marshalling + Writing): {:.2?}",
        before.elapsed()
    );
}
