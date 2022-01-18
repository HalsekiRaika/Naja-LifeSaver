use chrono::Local;
use yansi::{Paint, Color};

/// A structure for using Logger.
/// 
/// By using new, you can get ready to use it.
/// 
/// ## Usage
/// 
/// ```rust
/// let logger = Logger::new(Option<&'static str>);
/// 
/// //Infomation
/// logger.pinfo("infomation");
/// 
/// //Caution
/// logger.pcaut("caution");
/// 
/// //Warning
/// logger.pwarn("warning");
/// 
/// //Error
/// logger.perr("error");
/// ```

#[derive(Debug, Clone)]
pub struct Logger { thread_name: Option<&'static str> }
pub struct Instant(String);

#[allow(dead_code)]
impl Logger {
    pub fn new(thread_name: Option<&'static str>) -> Self {
        Self { thread_name }
    }

    fn p(&self, level: &str, level_color: Color, msg: &str) {
        let thread = match self.thread_name {
            Some(value) => format!("[ {:<12} ] ", Paint::green(value)),
            None => "".to_string()
        };

        println!("[{}] [ {:^4} ] {}{}", 
            Local::now().format("%H:%M:%S - %m/%d"), 
            Paint::new(format!("{:<5}", level)).fg(level_color),
            thread, msg)
    }

    pub fn info(&self, msg: &str) {
        self.p("Info", Color::Cyan, msg);
    }
    
    pub fn caut(&self, msg: &str) {
        self.p("Caut", Color::Yellow, msg);
    }

    pub fn warn(&self, msg: &str) {
        self.p("Warn", Color::Magenta, msg);
    }
    
    pub fn error(&self, msg: &str) {
        self.p("Error", Color::Red, msg);
    }

    pub fn debug(&self, msg: &str) {
        self.p("Debug", Color::Magenta, msg);
    }

}

impl Instant {
    pub fn t_name(thread: impl Into<String>) -> Self {
        Self(thread.into())
    }

    pub fn out(&self, status: impl Into<String>, colorize: yansi::Color, msg: impl Into<String>) {
        println!("[{}] [ {:^4} ] [ {:<12} ] {}",
                 Local::now().format("%H:%M:%S - %m/%d"),
                 Paint::new(format!("{:<5}", status.into())).fg(colorize),
                 Paint::green(&self.0), msg.into())
    }
}