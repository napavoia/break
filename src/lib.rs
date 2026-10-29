mod color;
mod context;
mod error;
mod lua;

pub mod game;
pub mod math;
pub use color::Color;

pub use context::{Context, builder::*};
use game::Status;
use std::path::Path;

pub fn run<P: AsRef<Path>>(ctx: &mut Context, script: P) -> Status {
    tracing::info!(target: "Break", "Run script {:?}", script.as_ref());
    ctx.run(script)
}

/// Downloads the `definition.lua` file (LuaLS annotations) from github
/// and saves as "definition.lua".
///
/// Uses the system `curl` with `-L -f -o`: follows GitHub redirects,
/// fails on HTTP errors, and writes the response to a file. An existing
/// file at `definition.lua` is overwritten;
///
/// # Errors
///
/// Returns an `io::Error` if:
/// * `curl` is missing or fails to start;
/// * `curl` exits with an error (network, 404, etc.);
pub fn download_lua_definition() -> std::io::Result<()> {
    use std::process::Command;
    let url = "https://raw.githubusercontent.com/napavoia/break/refs/heads/main/definition.lua";
    let dest = "definition.lua";

    let status = Command::new("curl")
        .args(["-L", "-f", "-o", dest, url])
        .status()?;

    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("curl failed with status: {status}"),
        ));
    }

    tracing::info!(target: "Break", "Definition file download saved to {:?}", Path::new(dest));
    Ok(())
}
