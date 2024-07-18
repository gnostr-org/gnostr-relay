async fn heavy_computation() {
    tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
    println!("heavy computation finished");
}

async fn light_computation() {
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    println!("light computation finished");
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
