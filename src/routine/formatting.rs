use colored::Colorize;

pub trait MessagesFormatting {
    fn into_hint(self) -> String;
}

impl MessagesFormatting for String {
    fn into_hint(self) -> String {
        format!("  * {}", self.bright_blue())
    }
}
