use super::opcodes::get_mapping;
use crate::{extractors::config_builder::VMConfig, utils::logger::Logger};
pub struct VM<'a> {
    pub logger: Logger,
    pub mem: Vec<MemoryPoint<'a>>,
    pub pointer: usize,
    pub bytecode: &'a str,
    pub cnfg: &'a VMConfig,
    enc: u64,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum MemoryPoint<'a> {
    Undefined,
    String(&'a str),
    Opcode(fn(&mut VM<'a>)),
}

impl VM<'_> {
    pub fn from(cnfg: &VMConfig) -> VM<'_> {
        let mem: Vec<MemoryPoint> = (0..255).map(|_| MemoryPoint::Undefined).collect();

        let mut v = VM {
            logger: Logger::new("VM"),
            pointer: 0,
            mem: mem,
            bytecode: &cnfg.bytecodes.init,
            cnfg,
            enc: cnfg.magic_bits.start_enc,
        };
        v.setup();
        return v;
    }

    fn setup(&mut self) {
        let opcodes = get_mapping();
        for key in self.cnfg.registers.keys() {
            let val = self.cnfg.registers.get(key).unwrap();

            let op = opcodes.get(key.as_str());
            if op.is_some() {
                self.logger.debug(&format!("handler: {} -> {}", val, key));
                self.mem[*val as usize] = MemoryPoint::Opcode(*op.unwrap())
            } else if key == "VMDATA" {
                self.logger.debug(&format!("VMData: {} -> {}", val, key));
            } else if key.len() == 2 {
                self.logger.debug(&format!("Global: {} -> {}", val, key));
            }
        }
    }

    fn read(&mut self) -> u64 {
        let sub: i64 = (self.cnfg.magic_bits.opcode_enc + 256).try_into().unwrap();

        self.pointer += 1;
        let next = self.bytecode.chars().nth(self.pointer);

        if next.is_none() {
            self.logger.error("Could not read() next");
            return 0;
        }
        // println!(
        //     "[debug] {:?}, {:?}, {:?}, {:?}, {:?}",
        //     next,
        //     self.pointer,
        //     next.unwrap(),
        //     next.unwrap() as i64,
        //     sub
        // );
        return self.enc ^ ((next.unwrap() as i64 - sub) & 255) as u64;
    }

    fn calc_enc(&mut self, op: usize) {
        let j = self.enc + op as u64;

        let v: u64;
        let encs = &self.cnfg.magic_bits.enc;
        if encs.len() == 3 {
            v = j * j * encs[0] + encs[1] * j + encs[2]
        } else if encs.len() == 2 {
            v = encs[0] * j + encs[1]
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
            self.logger.debug(&format!(
                "Stepping in VM (opcode={:?}, index={}, enc={})",
                opcode, next_index, self.enc
            ));
            match opcode {
                MemoryPoint::Opcode(handler) => {
                    let _ = handler(self);
                }
                _ => {
                    self.logger.error("Expected opcode, but not an opcode");
                    break;
                }
            }
            break;
        }
    }

    pub fn run_init(&mut self) {
        self.logger.debug("Running VM (init)");
        self.bytecode = &self.cnfg.bytecodes.init.as_str();
        self.pointer = 0;
        self.run();
    }
    pub fn run_main(&mut self) {
        self.logger.debug("Running VM (main)");
        self.bytecode = &self.cnfg.bytecodes.main.as_str();
        self.pointer = 0;
        self.run();
    }
}
