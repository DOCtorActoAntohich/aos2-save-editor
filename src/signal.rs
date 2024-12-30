use anyhow::Context;

#[derive(Debug)]
pub struct TerminationSignal;

pub async fn termination_signal() -> anyhow::Result<TerminationSignal> {
    // i hate windows i hate windows i hate windows.
    let mut ctrl_close =
        tokio::signal::windows::ctrl_close().context("Failed to setup Ctrl+Close handler")?;
    let mut ctrl_break =
        tokio::signal::windows::ctrl_break().context("Failed to setup Ctrl+Break handler")?;

    tokio::select! {
        _ = ctrl_close.recv() => {
            Ok(TerminationSignal)
        }
        _ = ctrl_break.recv() => {
            Ok(TerminationSignal)
        }
    }
}
