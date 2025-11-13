use std::error::Error;
use csv::Reader;
use serde::{Deserialize, Serialize, Serializer};

// CSV status values
#[derive(Debug)]
pub enum TaskStatus {
    Completed, // CSV label
    Failed,    // CSV label
}

// Force plain string output
impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TaskStatus::Completed => serializer.serialize_str("Completed"),
            TaskStatus::Failed => serializer.serialize_str("Failed"),
        }
    }
}

// Row written to results
#[derive(Debug, Serialize)]
pub struct TaskResult {
    pub task_id: u64,
    pub final_status: TaskStatus, 
    pub error_info: String, 
}

// Task input row
#[derive(Debug, Deserialize, Clone)]
pub struct InputTask {
    pub task_id: u64,
    pub task_type: String, 
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let file_path = std::env::args().nth(1).expect("Usage: cargo run -- <tasks.csv path>");
    
    let mut reader: Reader<std::fs::File> = csv::Reader::from_path(file_path)?;

    let mut unique_task_ids: std::collections::HashSet<u64> = std::collections::HashSet::new();
    
    for result in reader.deserialize() {
        let record: InputTask = result?;
        unique_task_ids.insert(record.task_id);
    }

    // Log via stderr to keep stdout clean
    eprintln!("Processing {} unique tasks concurrently...", unique_task_ids.len());
    
    // Spawn all tasks
    let mut handles = Vec::new();
    
    for task_id in unique_task_ids {
        let handle = tokio::spawn(async move {
            (task_id, execute_task(task_id).await)
        });
        handles.push(handle);
    }
    
    // Collect completions
    let mut results: Vec<TaskResult> = Vec::new();
    
    for handle in handles {
        let (task_id, task_result) = handle.await
            .map_err(|e| format!("Task execution error: {}", e))?;
        
        match task_result {
            Ok(()) => {
                results.push(TaskResult {
                    task_id,
                    final_status: TaskStatus::Completed,
                    error_info: String::new(),
                });
            }
            Err(error_msg) => {
                results.push(TaskResult {
                    task_id,
                    final_status: TaskStatus::Failed,
                    error_info: error_msg,
                });
            }
        }
    }
    
    // Emit CSV
    let mut writer = csv::Writer::from_writer(std::io::stdout());
    for result in results {
        writer.serialize(result)?;
    }
    writer.flush()?;

    Ok(())
}

async fn execute_task(task_id: u64) -> Result<(), String> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://httpbin.org/get")
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("HTTP request returned status: {}", response.status()));
    }
    
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    
    eprintln!("Task {} completed successfully", task_id);
    
    Ok(())
}