use reqwest;
use std::time::Instant;
use tokio;

#[no_mangle]
pub extern "C" fn threaded_call(times: u32) -> u32 {
    let mut handles: Vec<std::thread::JoinHandle<_>> = Vec::new();
    let start = Instant::now();
    for i in 1..times {
        handles.push(std::thread::spawn(move || {
            println!("\nthread {}", i);
            match brands_api() {
                Ok(_) => {
                    println!("\n API call completed for thread {}\n", i);
                }
                Err(error) => {
                    println!("\n {} \n", error);
                }
            }
        }))
    }
    for handle in handles {
        match handle.join() {
            Ok(_) => {
                println!("\n Thread completed.\n");
            }
            Err(error) => {
                println!("\n{:?}\n", error);
            }
        }
    }
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
    0_u32
}

#[tokio::main]
async fn brands_api() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://api.box8.co.in/brands").await?;

    // println!("Status: {}", res.status());

    let body = res.text().await?;

    println!("\nBody:\n{}", body);
    Ok(())
}
