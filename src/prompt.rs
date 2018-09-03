
use termion::{self, color, style};

pub fn prompt() -> String {
    return format!("{}ImperatorJohannes{} ~$", color::Fg(color::Red), color::Fg(color::Reset));
}
