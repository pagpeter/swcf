use core::str;
use std::fs;

use super::opcodes;
use crate::{extractors::config_builder::VMConfig, utils::logger::Logger};
pub struct VM<'a> {
    pub logger: Logger,
    pub mem: Vec<MemoryPoint<'a>>,
    pub pointer: usize,
    pub bytecode: Vec<u8>,
    pub cnfg: &'a VMConfig,
    pub enc: u64,
    pub logs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum MemoryPoint<'a> {
    Undefined,
    String(&'a str),
    Opcode(fn(&mut VM<'a>)),
}

fn base64decode(s: &str) -> Vec<u8> {
    return base64::Engine::decode(&base64::prelude::BASE64_STANDARD, s).unwrap();
}

impl VM<'_> {
    pub fn from(cnfg: &VMConfig) -> VM<'_> {
        let mem: Vec<MemoryPoint> = (0..256).map(|_| MemoryPoint::Undefined).collect();

        let mut v = VM {
            logger: Logger::new("VM"),
            pointer: 0,
            mem: mem,
            bytecode: vec![],
            cnfg,
            enc: cnfg.magic_bits.start_enc,
            logs: vec![],
        };
        v.setup();
        return v;
    }

    fn setup(&mut self) {
        let opcodes = opcodes::get_mapping();
        for key in self.cnfg.registers.keys() {
            let val = self.cnfg.registers.get(key).unwrap();

            let op = opcodes.get(key.as_str());
            if op.is_some() {
                self.logger.debug(&format!("reg_{} = {}", val, key));
                self.logs.push(format!("reg_{} = {}", val, key));
                self.mem[*val as usize] = MemoryPoint::Opcode(*op.unwrap())
            } else if key == "VMDATA" {
                self.logger.debug(&format!("VMData: {} -> {}", val, key));
            } else if key.len() == 2 {
                self.logger.debug(&format!("Global: {} -> {}", val, key));
            }
        }
    }

    pub fn push_instruction(&mut self, code: &str, debug: &str) {
        println!("[Opcode] {} // {}", code, debug);
        self.logs.push(format!("{}; // {}", code, debug));
    }

    pub fn read(&mut self) -> u64 {
        let sub: i64 = (self.cnfg.magic_bits.opcode_enc + 256).try_into().unwrap();

        let next = self.bytecode[self.pointer];
        self.pointer += 1;

        // if next.is_none() {
        //     self.logger.error("Could not read() next");
        //     return 0;
        // }
        let after_enc = self.enc ^ ((next as i64 - sub) & 255) as u64;
        // println!(
        //     "[read debug] next={:?}, pointer={:?}, sub={:?}, after_enc={:?}",
        //     next, self.pointer, sub, after_enc
        // );
        return after_enc;
    }

    fn calc_enc(&mut self, op: usize) {
        let j = self.enc + op as u64;

        let v: u64;
        let encs = &self.cnfg.magic_bits.enc;
        if encs.len() == 3 {
            // v = j * j * encs[0] + encs[1] * j + encs[2];
            v = encs[0] + encs[1] + (j * j) + j + encs[2];
        } else if encs.len() == 2 {
            v = encs[0] * j + encs[1];
        } else {
            self.logger.error("Could not calc_enc()");
            return;
        }

        self.enc = v & 255;
    }

    fn run(&mut self) {
        loop {
            let next_index = self.read() as usize;

            self.calc_enc(next_index);

            let opcode = self.mem[next_index].clone();

            // self.logger.debug(&format!(
            //     "Stepping in VM (opcode={:?}, index={}, enc={})",
            //     opcode, next_index, self.enc
            // ));
            match opcode {
                MemoryPoint::Opcode(handler) => {
                    let _ = handler(self);
                }
                _ => {
                    self.logger.error(&format!(
                        "Expected opcode, but not an opcode (reg_{})",
                        next_index
                    ));
                    println!("{} {:?}", next_index, self.mem[next_index]);
                    break;
                }
            }
        }
    }

    pub fn run_init(&mut self) {
        self.logger.debug("Running VM (init)");
        self.bytecode = base64decode(&self.cnfg.bytecodes.init);
        self.pointer = 0;
        self.run();
        fs::write("./data/traces.txt", format!("{}", self.logs.join("\n")))
            .expect("Could not write traces");
    }
    pub fn run_main(&mut self) {
        self.logger.debug("Running VM (main)");
        self.bytecode = self.cnfg.bytecodes.main.as_bytes().to_vec();
        self.pointer = 0;
        self.run();
        fs::write("./data/traces.txt", format!("{}", self.logs.join("\n")))
            .expect("Could not write traces");
    }
}
