use swccf::extract_required;
use swccf::logger;
use swccf::requests;

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

    log.success(format!("Parsed ChallengeData: {}", challenge_data.c_ray));
    log.debug("Getting init script".to_owned());

    let script = requests::get_script(&challenge_data);
    if script.is_err() {
        log.error("Could not get init script".to_owned())
    }
    let script_result = script.unwrap();
    log.success(format!("Got script!"));
    log.debug(format!("Parsing script"));
    let script_data = extract_required::parse_script(&script_result);
    log.debug("Submitting init challenge".to_owned());
    let bytecode = requests::submit_init(&challenge_data, &script_data);

    if bytecode.is_err() {
        log.error("Could not submit init challenge".to_owned())
    }
    log.success(format!("Got main challenge!"));
    println!("{}", bytecode.unwrap());
}
