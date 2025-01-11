use std::fmt::{Display, LowerHex};

#[derive(derive_more::From)]
pub struct HexyNumber<T>(T);

impl<T> Display for HexyNumber<T>
where
    T: Display + LowerHex,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} / {:#x}", self.0, self.0)
    }
}
