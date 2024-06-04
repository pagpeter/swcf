use crate::{logger::Logger, traversals::config_builder::VMConfig};

pub struct VM<'a> {
    logger: Logger,
    mem: Vec<MemoryPoint>,
    pointer: usize,
    bytecode: &'a str,
    cnfg: &'a VMConfig,
}

enum MemoryPoint {
    Undefined,
    String(String),
    Opcode(Box<dyn Fn()>),
}

impl VM<'_> {
    pub fn from(cnfg: &VMConfig) -> VM<'_> {
        let mem: Vec<MemoryPoint> = (0..255).map(|_| MemoryPoint::Undefined).collect();

        return VM {
            logger: Logger::new("VM"),
            pointer: 0,
            mem: mem,
            bytecode: &cnfg.bytecodes.init,
            cnfg,
        };
    }

    fn run(&mut self) {}

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
