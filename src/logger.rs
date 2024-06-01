use chrono::Local;

pub struct Logger {
    prefix: String,
}

fn get_timestamp() -> String {
    let time = Local::now();
    time.format("[%H:%M:%S:%3f]").to_string()
}

impl Logger {
    pub fn debug(&self, text: String) {
        println!(
            "\u{1B}[95m{} [{}]\u{001b}[0m {}",
            get_timestamp(),
            self.prefix,
            text
        )
    }
    pub fn error(&self, text: String) {
        println!(
            "\u{1B}[91m{} [{}]\u{001b}[0m {}",
            get_timestamp(),
            self.prefix,
            text
        )
    }
    pub fn success(&self, text: String) {
        println!(
            "\u{1B}[92m{} [{}]\u{001b}[0m {}",
            get_timestamp(),
            self.prefix,
            text
        )
    }
}

pub fn get_logger(prefix: String) -> Logger {
    return Logger { prefix };
}
