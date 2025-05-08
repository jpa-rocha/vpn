mod config;

use anyhow::{Context, Result};
use clap::Parser;
use config::Config;
use std::process::Command;

/// Simple program to connect to vpn
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Action to take
    #[arg(short, long)]
    action: String,
}

fn main() -> Result<()> {
    let cfg: Config = confy::load("vpn", "vpn").context("failed to read config")?;
    let args = Args::parse();
    vpn_cmd(args.action, cfg)?;
    Ok(())
}

fn vpn_cmd(command: String, cfg: Config) -> Result<()> {
    let result = match command.as_str() {
        "connect" => Command::new("forticlient")
            .arg("vpn")
            .arg("connect")
            .arg(cfg.vpn)
            .arg("-w")
            .output(),
        "disconnect" => Command::new("forticlient")
            .arg("vpn")
            .arg("disconnect")
            .output(),
        _ => anyhow::bail!("invalid command: `{}`", command),
    };

    let output = result.context("failed to execute forticlient command")?;
    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    } else {
        anyhow::bail!(
            "command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
