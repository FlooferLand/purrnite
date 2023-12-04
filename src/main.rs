#![windows_subsystem = "windows"]
mod raven;

use raven::*;
mod fortnite;
mod check;
mod popup;

use tokio_schedule::{every, Job};
use crate::popup::popup;

pub async fn get_io_key() -> String {
    match std::env::var("IO_KEY") {
        Ok(key) => key.trim().to_string(),
        Err(err) => {
            let title = "Did not find the FortniteAPI.IO key.".to_string();
            let msg = "Read the Purrnite README.md!".to_string();
            popup(title.clone(), format!("{title}\n{msg}")).await;
            panic!("\n{title} {msg}\n{err}")
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    #[cfg(windows)] native_windows_gui::init().expect("Failed to initialise Windows GUI");
    raven().await;

    // ---

    let thread_periodic = every(4).hours().perform(raven);
    let _ = tokio::spawn(thread_periodic).await;

    // TODO: Make it also check 5 minutes after the shop updates (midnight i think?)
    let thread_scheduled = every(1).day().at(02, 05, 00).perform(raven);
    let _ = tokio::spawn(thread_scheduled).await;
}
