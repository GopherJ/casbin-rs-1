use crate::logger::Logger;

use log::{error, info};

#[derive(Default)]
pub struct DefaultLogger {
    enabled: bool,
}

impl Logger for DefaultLogger {
    fn enable_log(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn print(&self, rvals: Vec<String>, authorized: bool, is_cached: bool) {
        if !self.is_enabled() {
            return;
        }

        let mut text: String = String::from(if is_cached { "[CACHE]" } else { "" });

        if authorized {
            text = format!(
                "{} [Request: {:?} ---> Response: {}]",
                text,
                rvals.join(", "),
                true
            );
            info!("{}", text);
        } else {
            text = format!(
                "{} [Request: {:?} ---> Response: {}]",
                text,
                rvals.join(", "),
                false
            );
            error!("{}", text);
        }
    }
}
