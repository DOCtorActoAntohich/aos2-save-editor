use std::fmt::Display;

use crate::fmt::HexyNumber;

#[derive(Debug, Clone, Copy)]
pub struct ByteDifference {
    pub position: usize,
    pub previous: u8,
    pub current: u8,
}

#[derive(derive_more::From)]
pub struct ByteDifferenceTable<'a>(&'a [ByteDifference]);

impl Display for ByteDifferenceTable<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return Ok(());
        }

        writeln!(
            f,
            "| {:<16} | {:<16} | {:<16}",
            "Position", "Before", "After"
        )?;

        self.0.iter().try_for_each(|&diff| {
            writeln!(
                f,
                "| {:<16} | {:<16} | {:<16}",
                HexyNumber::from(diff.position).to_string(),
                HexyNumber::from(diff.previous).to_string(),
                HexyNumber::from(diff.current).to_string()
            )
        })?;

        Ok(())
    }
}
