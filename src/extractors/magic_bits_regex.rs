use super::config_builder::VMConfig;
use crate::utils::utils;
impl VMConfig {
    fn find_start_enc(&mut self, script: &str) {
        let caps = utils::find_from_multiple_regexes(
            script,
            vec![
                r"atob\(.\),(\d+)",
                r"atob,.\),(\d+?),",
                r"atob\(.\),\n.+?(\d+?),",
            ],
        );
        if caps.is_none() {
            println!("[!] Could not get start enc")
        } else {
            self.magic_bits.start_enc = caps.unwrap()[1].parse().unwrap();
        }
    }
    fn find_opcode_enc(&mut self, script: &str) {
        let caps = utils::find_from_multiple_regexes(
            script,
            vec![r"\+\+\)-(\d{1,}),256", r"\+\+\) - (\d{1,}), 256"],
        );
        if caps.is_none() {
            println!("[!] Could not opcode enc")
        } else {
            self.magic_bits.opcode_enc = caps.unwrap()[1].parse().unwrap();
        }
    }
    pub fn find_all_enc(&mut self, script: &str) {
        self.find_opcode_enc(script);
        self.find_start_enc(script)
    }
}
