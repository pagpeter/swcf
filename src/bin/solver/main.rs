use std::fs;

use swccf::extract_required;
use swccf::logger::Logger;
use swccf::requests;
use swccf::traversals::deobfuscate_script::deobfuscate;
use swccf::traversals::utils;
use swccf::vm::vm::VM;

fn main() {
    let log = Logger::new("MAIN");

    log.debug("Getting initial HTML");

    let mut session = requests::SolvingSession::new("cfschl.peet.ws", false);

    let text = session.get_page();

    if text.is_err() {
        log.error("Could not get initial HTML")
    }

    let html_result = text.unwrap();
    log.debug("Got result HTML, parsing it...");

    let challenge_data = extract_required::parse_challenge_data(&html_result).unwrap();

    session.cnfg.chl_data = challenge_data;

    log.success(&format!(
        "Parsed ChallengeData: {}",
        &session.cnfg.chl_data.c_ray
    ));
    log.debug("Getting init script");

    let script = session.get_script();
    if script.is_err() {
        log.error("Could not get init script")
    }
    let script_result = script.unwrap();
    log.success(&format!("Got script!"));
    log.debug(&format!("Parsing script"));
    let script_data = extract_required::parse_script(&script_result);

    let deobbed_script = deobfuscate(&mut session.cnfg, &script_result);

    println!("[*] Writing deobfuscated script to file (./data/input_out.js)");
    fs::write("./data/input_out.js", deobbed_script).expect("Could not write file");
    if session.cnfg.payloads.init.len() < 30 {
        log.error("[error] Could not find init keys");
        return;
    }
    log.debug("Submitting init challenge");
    let bytecode = session.submit_init(&script_data);

    if bytecode.is_err() {
        log.error("Could not submit init challenge")
    }
    let main = bytecode.unwrap();
    if main.len() < 20 {
        log.error("Error getting main bytecode:");
        log.error(&main)
    }
    log.success("Got main challenge bytecode!");
    session.cnfg.bytecodes.main = utils::decrypt_response(&main, &session.cnfg.chl_data.c_ray);

    println!("[*] Writing extracted vm config to file (./data/vm_config.json)");
    let json = serde_json::to_string_pretty(&session.cnfg);
    fs::write("./data/vm_config.json", json.unwrap()).expect("Could not write file");

    let mut vm = VM::from(&session.cnfg);
    vm.run_init();
}
