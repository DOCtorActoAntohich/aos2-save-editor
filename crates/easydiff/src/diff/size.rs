use std::fmt::Display;

use crate::fmt::HexyNumber;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SizeDifference {
    pub previous: usize,
    pub current: usize,
}

#[derive(Debug, Clone, Copy, derive_more::From)]
pub struct SizeDifferenceTable<'a>(&'a SizeDifference);

impl Display for SizeDifference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} vs {}",
            HexyNumber::from(self.previous),
            HexyNumber::from(self.current)
        )
    }
}

impl Display for SizeDifferenceTable<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Size difference")?;
        writeln!(f, "| {:<16} | {:>16} |", "Previous", "Current")?;
        writeln!(
            f,
            "| {:<16} | {:>16} |",
            HexyNumber::from(self.0.previous).to_string(),
            HexyNumber::from(self.0.current).to_string()
        )
    }
}
