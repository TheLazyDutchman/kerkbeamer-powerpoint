use std::{process::Command, thread::sleep, time::Duration};

use transparent::{CommandExt, TransparentRunner};

fn main() {
    let mut child = Command::new("POWERPNT.EXE").spawn_transparent(&TransparentRunner::new()).unwrap();

    sleep(Duration::from_secs(2));

    println!("Spawned");

    let code = child.wait().unwrap();
    println!("PowerPoint exited with code: {code}");
}