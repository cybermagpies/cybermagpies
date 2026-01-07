use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
struct DeviceReport {
    device_name: String,
    logs_found: String,
    risk_level: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Magpie Agent Started...");

    // 1. COLLECT: Get Windows Logs (We look for errors in 'Application')
    // This command uses Windows' native 'wevtutil' tool
    let output = Command::new("wevtutil")
        .args(&["qe", "Application", "/c:3", "/f:text", "/q:*[System[(Level=2)]]"])
        .output();

    // Handle case where wevtutil might fail (e.g. permission issues)
    let log_data = match output {
        Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        Err(_) => "Could not access logs (Admin rights needed?)".to_string(),
    };
    
    // 2. ANALYZE: Simple Keyword Check
    let risk_level = if log_data.contains("Error") {
        "Medium".to_string()
    } else if log_data.is_empty() {
        "Safe".to_string()
    } else {
        "Low".to_string()
    };

    println!("Analysis Complete. Risk: {}", risk_level);

    // 3. REPORT: Send to Server
    let client = reqwest::Client::new();
    let report = DeviceReport {
        device_name: hostname::get()?.to_string_lossy().into_owned(),
        logs_found: log_data.chars().take(200).collect(), // First 200 chars only
        risk_level,
    };

    // Note: ensure port 3000 matches your server
    let res = client.post("http://127.0.0.1:3000/submit_report")
        .json(&report)
        .send()
        .await?;

    println!("Server Response: {:?}", res.status());

    Ok(())
}
