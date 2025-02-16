use std::time::Duration;

pub trait Theme {
    fn new() -> Self;
    fn format(&self, dur: &Duration) -> String;
}
