use std::{io::{stdin, Write}, path::PathBuf, process::Command};

use clap::Parser;
use path_clean::PathClean;
use transparent::{CommandExt, TransparentRunner};

#[derive(Parser)]
struct Args {
    path: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    assert!(args.path.exists(), "Geen PowerPoint presentatie gevonden op `{:?}`", args.path);
    let controller_prefix: PathBuf = "../PowerPointController/bin/Release".parse().unwrap();

    let server_path = controller_prefix.join("PowerPointController.exe");

    let absolute_path = if args.path.is_absolute() {
        args.path
    } else {
        std::env::current_dir()?.join(args.path)
    }.clean();

    let mut server = Command::new(server_path).arg(absolute_path).spawn_transparent(&TransparentRunner::new())?;

    // let websocket = reqwest_websocket::websocket("ws://localhost:8181").await?;

    let mut line = String::new();
    stdin().read_line(&mut line)?;

    server.stdin.as_mut().unwrap().write_all(b"\n")?;

    server.wait_with_output()?;

    Ok(())
}