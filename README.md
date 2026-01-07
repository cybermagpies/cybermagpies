# CyberMagpies

CyberMagpies is a distributed security monitoring system built in Rust. It automates the process of collecting Windows Event Logs from multiple computers and reporting them to a central server for analysis.

The project is designed as a lightweight prototype for an Endpoint Detection and Response (EDR) system. It focuses on speed, memory safety, and ease of deployment.

## Project Architecture

The system is split into two main components that communicate over HTTP.

### 1. The Magpie (Client Agent)
The client application runs on the target Windows machine. Its job is to act as a scout. It interfaces directly with the Windows operating system using the `wevtutil` command to extract recent logs from the Application and Security channels.

Instead of sending raw data, the agent performs a quick local analysis. It looks for specific error patterns or warning signs, calculates a risk level, and packs the information into a JSON report.

### 2. The Nest (Server)
The server acts as the command center. It is built using the Axum web framework and runs asynchronously. It listens for incoming reports from the agents, validates the data, and displays the alerts in real-time.

## Why Rust?

We chose Rust for this project because it provides memory safety without garbage collection. This allows the "Magpie" agent to be compiled into a very small, standalone executable that can run on any Windows machine without needing complex dependencies installed.

## Technical Stack

* **Language:** Rust
* **Server Framework:** Axum & Tokio
* **Client Requests:** Reqwest
* **Data Handling:** Serde (JSON serialization)
* **System Interface:** Windows Process Command

## How to Run

### Prerequisites
You will need Rust installed on your machine. Since the agent interacts with Windows logs, the client must be run on a Windows environment (Windows 10 or 11).

### 1. Setup
Clone the repository and enter the directory:
```bash
git clone [https://github.com/cybermagpies/cybermagpies.git](https://github.com/cybermagpies/cybermagpies.git)
cd cybermagpies
