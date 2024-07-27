use std::fs;

use swccf::{extractors::config_builder::VMConfig, vm::vm::VM};

fn main() {
    let cnfg: VMConfig;

    let cached = fs::read("./data/vm_config.json");
    if cached.is_ok() {
        let cached = cached.unwrap();
        let str = std::str::from_utf8(&cached).unwrap();
        let res = serde_json::from_str(str.into());
        if res.is_ok() {
            cnfg = res.unwrap();
        } else {
            println!("Error loading cached vm_config: {:#?}", res.err());
            return;
        }
    } else {
        println!("No vm_config.json file found");
        return;
    }

    let mut vm = VM::from(&cnfg);
    vm.run_init();
    vm.run_main()
}
