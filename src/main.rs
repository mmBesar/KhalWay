use serde::Serialize;
use std::env;
use std::process::Command;

#[derive(Serialize)]
struct WaybarOutput {
    text: String,
    tooltip: String,
    class: String,
    icon: String,
}

async fn fetch_khal_events(args: Vec<String>) -> String {
    let output = Command::new("khal")
        .arg("list")
        .args(args)
        .output()
        .expect("Failed to execute khal");

    String::from_utf8_lossy(&output.stdout).to_string()
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let events_output = fetch_khal_events(args).await;

    let has_events = !events_output.trim().is_empty();

    let tooltip = if has_events {
        events_output
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<&str>>()
            .join("\n")
    } else {
        "No upcoming events".to_string()
    };

    // NerdFont icons
    let icon = if has_events {
        "󱅫" // Custom icon if events are present
    } else {
        "" // Default calendar icon
    };

    let output = WaybarOutput {
        text: icon.to_string(), // Only icon shown on bar
        tooltip,
        class: if has_events { "notifications" } else { "default" }.to_string(),
        icon: icon.to_string(),
    };

    println!("{}", serde_json::to_string(&output).unwrap());
}
