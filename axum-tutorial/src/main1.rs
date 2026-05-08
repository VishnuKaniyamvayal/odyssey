use tokio::time::{ interval };
use std::time::Duration;


#[tokio::main]
async fn main(){
    let tps: u64 = 1000 / 128;
    let mut time = interval(Duration::from_millis(tps));
    loop {
        time.tick().await;
        println!("tick");
    }
}