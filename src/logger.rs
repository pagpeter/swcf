use chrono::Local;

pub struct Logger {
    prefix: String,
}

fn get_timestamp() -> String {
    let time = Local::now();
    time.format("[%H:%M:%S:%3f]").to_string()
}

impl Logger {
    pub fn new(prefix: &str) -> Logger {
        return Logger {
            prefix: prefix.to_string(),
        };
    }
    pub fn debug(&self, text: &str) {
        println!(
            "\u{1B}[95m{} [{}]\u{001b}[0m {}",
            get_timestamp(),
            self.prefix,
            text
        )
    }
    pub fn error(&self, text: &str) {
        println!(
            "\u{1B}[91m{} [{}]\u{001b}[0m {}",
            get_timestamp(),
            self.prefix,
            text
        )
    }
    pub fn success(&self, text: &str) {
        println!(
            "\u{1B}[92m{} [{}]\u{001b}[0m {}",
            get_timestamp(),
            self.prefix,
            text
        )
    }
}
