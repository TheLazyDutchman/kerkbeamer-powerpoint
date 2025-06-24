use std::{io::{stdin, Read, Write}, path::PathBuf, process::Command, thread::sleep, time::Duration};
use std::sync::Arc;
use std::sync::Mutex;

use anyhow::anyhow;
use clap::Parser;
use tokio::{task::spawn_blocking, time::timeout};
use transparent::{CommandExt, TransparentRunner};

#[derive(clap::Parser, Debug)]
struct CommandArgs {
    #[command(subcommand)]
    command: SlideShowCommand
}

#[derive(clap::Subcommand, Debug)]
enum SlideShowCommand {
    /// Open presentation at [`path`]
    Open {
        path: PathBuf
    },

    /// Go to slide at index=[`slide`]
    /// 
    /// Slides are zero-indexed
    Goto {
        slide: usize
    },

    /// Quit slideshow, and exit
    Quit,
}

#[derive(clap::Parser)]
struct Args {
    path: PathBuf
}

fn wait_result(stdout: Arc<Mutex<std::process::ChildStdout>>, stderr: Arc<Mutex<std::process::ChildStderr>>) -> anyhow::Result<String> {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let stdout = spawn_blocking(move || {
            let mut buf = [0; 256];
            let len = stdout.lock().unwrap().read(&mut buf).unwrap();
            String::from_utf8_lossy(&buf[0..len]).to_string()
        });

        match timeout(Duration::from_millis(50), stdout).await {
            Ok(v) => v,
            Err(_) => {
                let stderr = spawn_blocking(move || {
                    let mut buf = [0; 256];
                    let len = stderr.lock().unwrap().read(&mut buf).unwrap();
                    String::from_utf8_lossy(&buf[0..len]).to_string()
                });
                timeout(Duration::from_millis(50), stderr).await?
            }
        }.map_err(Into::into)
    })
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut powerpoint_child = Command::new("POWERPNT.EXE").spawn_transparent(&TransparentRunner::new())?;

    sleep(Duration::from_secs(10));

    let mut slideshow_child = Command::new("node").arg(args.path).spawn_transparent(&TransparentRunner::new())?;
    sleep(Duration::from_secs(2));

	let mut child_stdin = slideshow_child.stdin.take().ok_or(anyhow!("Expected slideshow stdin"))?;
	let child_stdout = slideshow_child.stdout.take().ok_or(anyhow!("Expected slideshow stdout"))?;
	let child_stderr = slideshow_child.stderr.take().ok_or(anyhow!("Expected slideshow stderr"))?;
    let child_stdout = Arc::new(Mutex::new(child_stdout));
    let child_stderr = Arc::new(Mutex::new(child_stderr));

    println!("Spawned");

    wait_result(child_stdout.clone(), child_stderr.clone())?; // Get question out
    child_stdin.write_all("use powerpoint\n".as_bytes())?;
    wait_result(child_stdout.clone(), child_stderr.clone())?; // Get question out
    child_stdin.write_all("boot\n".as_bytes())?;

    println!("Started");

    let mut command = String::new();
    loop {
        let question = wait_result(child_stdout.clone(), child_stderr.clone())?;
        println!("{question}");
        command.clear();
        stdin().read_line(&mut command).unwrap();

        let args = command.trim().split(' ').collect::<Vec<_>>();
        let command = CommandArgs::try_parse_from(std::iter::once("").chain(args))?;

        match command.command {
            SlideShowCommand::Open { path } => todo!(),
            SlideShowCommand::Goto { slide } => todo!(),
            SlideShowCommand::Quit => break,
        }
    }

    let code = powerpoint_child.wait().unwrap();
    println!("PowerPoint exited with code: {code}");

    Ok(())
}