use std::process::{Command, ExitStatus};
use std::io::Result;

pub fn execute(exe: &str, args: &[&str]) -> Result<ExitStatus> {
    Command::new(exe).args(args).spawn()?.wait()
}

async fn heavy_computation() {
    tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
    println!("heavy computation finished");
    execute("gnostr-relay",&["-p", "1111"]);
}

async fn light_computation() {
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    println!("light computation finished");
    execute("gnostr-relay",&["-p", "1112"]);
}

#[tokio::main]
async fn main() {
    let heavy = tokio::task::spawn(heavy_computation());
    println!("computation started");
    let light = tokio::task::spawn(async move {
        for _ in 0..3 {
            light_computation().await;
        }
    });
    let (a, b) = tokio::join!(heavy, light);
    a.unwrap();
    b.unwrap();
}
