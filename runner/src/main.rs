use std::{io::{Read, Write}, path::PathBuf, process::{Command, Stdio}, sync::{Arc, Mutex}, time::Duration};

use tokio::{io::AsyncBufReadExt, time::sleep};
use transparent::{CommandExt, TransparentRunner};
use anyhow::anyhow;
use clap::Parser;

#[derive(Parser)]
struct Args {
    path: PathBuf
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut powerpoint = Command::new("POWERPNT.EXE").spawn_transparent(&TransparentRunner::new())?;
    sleep(Duration::from_secs(10)).await;

    let mut child = Command::new("node").arg(args.path).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;

    let mut stdin = child.stdin.take().expect("Expected child stdin");
    let mut stdout = child.stdout.take().expect("Expected child stdout");

    let (sender, mut receiver) = tokio::sync::mpsc::channel(4);
    let (end_sender, end_receiver) = tokio::sync::oneshot::channel();

    tokio::task::spawn_blocking(move || {
        loop {
            let mut buf = [0; 256];
            println!("Reading");
            let len = stdout.read(&mut buf).unwrap();
            println!("Read");

            let out = String::from_utf8_lossy(&buf[..len]).to_string();
            let out = out.trim().to_string();

            if end_receiver.is_empty() {
                sender.blocking_send(out.clone()).unwrap();
                println!("Read out ({}): {out}", out.len());
            } else {
                break;
            }
        }
    });

    let mut expect = async |expected: &'static str| {
        let value = receiver.recv().await.ok_or(anyhow!("Channel is closed")).unwrap();
        assert_eq!(value, expected);
    };

    expect("slideshow>").await;

    stdin.write_all("use powerpoint\n".as_bytes())?;

    expect("slideshow(powerpoint)>").await;

    stdin.write_all("boot\n".as_bytes())?;

    expect("\"OK\"").await;
    expect("slideshow(powerpoint)>").await;

    println!("Spawned");

    let mut repl = tokio::io::BufReader::new(tokio::io::stdin());
    
    let end = move || {
        end_sender.send(()).map_err(|_| anyhow!("Reader thread got closed")).unwrap();
        let _ = powerpoint.kill();
        let _ = child.kill();

        // TODO: Find a way to get this message to the process
        // stdin.write_all("quit\n".as_bytes()).unwrap();
        println!("Press enter to exit");
    };
    let end = Arc::new(Mutex::new(Some(end)));
    let end1 = end.clone();
    
    let exit = tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        if let Some(end) = end1.lock().unwrap().take() {
            end()
        }
    });

    let input = tokio::spawn(async move {
        loop {
            if end.lock().unwrap().is_none() { break; }

            let mut input = String::new();

            println!("Waiting for input...");
            repl.read_line(&mut input).await.unwrap();
            if ["exit", "q", "quit"].contains(&input.to_lowercase().trim()) {
                break;
            }
            println!("Got input");

            if input.trim().is_empty() { continue; }

            sleep(Duration::from_millis(100)).await;
            println!("Writing");
            stdin.write_all(input.as_bytes()).unwrap();
            println!("Written");
            sleep(Duration::from_millis(100)).await;
        }

        if let Some(end) = end.lock().unwrap().take() {
            end()
        }
    });

    tokio::select! {
        _ = input => {}
        _ = exit => {}
    }

    Ok(())
}