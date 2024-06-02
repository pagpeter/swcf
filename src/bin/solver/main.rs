use std::fs;

use swccf::extract_required;
use swccf::logger;
use swccf::requests;
use swccf::traversals::config_builder;
use swccf::traversals::deobfuscate_script::deobfuscate;

fn main() {
    let log: logger::Logger = logger::get_logger("main".to_string());

    log.debug("Getting initial HTML".to_owned());
    let text = requests::get_page();

    if text.is_err() {
        log.error("Could not get initial HTML".to_owned())
    }

    let html_result = text.unwrap();
    log.debug(format!("Got result HTML, parsing it...",));

    let challenge_data = extract_required::parse_challenge_data(&html_result).unwrap();

    let mut vm_config = config_builder::VMConfig::default();
    vm_config.chl_data = challenge_data;

    log.success(format!(
        "Parsed ChallengeData: {}",
        &vm_config.chl_data.c_ray
    ));
    log.debug("Getting init script".to_owned());

    let script = requests::get_script(&vm_config.chl_data);
    if script.is_err() {
        log.error("Could not get init script".to_owned())
    }
    let script_result = script.unwrap();
    log.success(format!("Got script!"));
    log.debug(format!("Parsing script"));
    let script_data = extract_required::parse_script(&script_result);

    let deobbed_script = deobfuscate(&mut vm_config, &script_result);

    println!("[*] Writing extracted vm config to file (./data/vm_config.json)");
    let json = serde_json::to_string_pretty(&vm_config);
    fs::write("./data/vm_config.json", json.unwrap()).expect("Could not write file");
    println!("[*] Writing deobfuscated script to file (./data/input_out.js)");
    fs::write("./data/input_out.js", deobbed_script).expect("Could not write file");

    log.debug("Submitting init challenge".to_owned());
    let bytecode = requests::submit_init(&vm_config.chl_data, &script_data);

    if bytecode.is_err() {
        log.error("Could not submit init challenge".to_owned())
    }
    log.success(format!("Got main challenge!"));
    println!("{}", bytecode.unwrap());
}
