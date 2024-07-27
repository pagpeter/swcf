use std::fs;

use swccf::extractors::extract_required;
use swccf::networking::requests;
use swccf::utils::deobfuscate_script::deobfuscate;
use swccf::utils::logger::Logger;
use swccf::utils::utils;
use swccf::vm::vm::VM;

const DEBUG: bool = false;

fn main() {
    let log = Logger::new("MAIN");

    log.debug("Getting initial HTML");

    let mut session = requests::SolvingSession::new("cfschl.peet.ws", DEBUG);

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
    let _ = fs::write("./data/solver_init_raw.js", &script_result);
    log.debug(&format!("Parsing script"));
    let script_data = extract_required::parse_script(&script_result);

    let deobbed_script = deobfuscate(&mut session.cnfg, &script_result);

    println!("[*] Writing deobfuscated script to file (./data/input_out.js)");
    fs::write("./data/input_out.js", deobbed_script).expect("Could not write file");
    if session.cnfg.payloads.init.len() < 30 {
        log.error("[error] Could not find init keys");
        return;
    }

    let key: &[u8] = script_data.key.as_bytes();
    let payload = extract_required::lz_compress(&session.cnfg.payloads.init, key);
    let c_ray = &session.cnfg.chl_data.c_ray;
    let body = format!("v_{}={}", c_ray, payload.replacen("+", "%2b", 1));

    log.debug("Submitting init challenge");
    let bytecode = session.submit_init(&script_data, body.clone());

    if bytecode.is_err() {
        log.error("Could not submit init challenge")
    }
    let main = bytecode.unwrap();
    if main.len() < 20 {
        log.error("Error getting main bytecode:");
        log.error(&main);
        log.error(&format!("Init payload: {}", session.cnfg.payloads.init));
        return;
    }
    log.success("Got main challenge bytecode!");

    session.cnfg.bytecodes.main = utils::decrypt_response(&main, &session.cnfg.chl_data.c_ray);
    session.cnfg.find_all_enc(&script_result);

    println!("[*] Writing extracted vm config to file (./data/vm_config.json)");
    let json = serde_json::to_string_pretty(&session.cnfg);
    fs::write("./data/vm_config.json", json.unwrap()).expect("Could not write file");

    let mut vm = VM::from(&session.cnfg);
    vm.run_init();
}
