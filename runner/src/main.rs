use std::{path::PathBuf, process::Command};

use clap::Parser;
use path_clean::PathClean;

#[derive(Parser)]
struct Args {
    path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    assert!(args.path.exists(), "Geen PowerPoint presentatie gevonden op `{:?}`", args.path);
    let controller_prefix: PathBuf = "../PowerPointController/bin/Release".parse().unwrap();

    let server_path = controller_prefix.join("PowerPointController.exe");

    let absolute_path = if args.path.is_absolute() {
        args.path
    } else {
        std::env::current_dir()?.join(args.path)
    }.clean();

    let server = Command::new(server_path).arg(absolute_path).spawn()?;

    Ok(())
}