use reqwest::Client;
use serde_json::json;
use std::error::Error;
use tokio::time::{sleep, Duration};

pub async fn send_message(assistant_id: &str, message: &str, api_key: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // Step 1: Create a new thread
    let thread_response = client
        .post("https://api.openai.com/v1/threads")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .header("OpenAI-Beta", "assistants=v2")
        .json(&json!({}))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let thread_id = thread_response["id"].as_str().ok_or("Missing thread ID")?;
    println!("Thread ID: {}", thread_id);

    // Step 2: Send a message to the assistant
    let message_url = format!("https://api.openai.com/v1/threads/{}/messages", thread_id);
    client
        .post(&message_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .header("OpenAI-Beta", "assistants=v2")
        .json(&json!({
            "role": "user",
            "content": message
        }))
        .send()
        .await?;

    // Step 3: Run the assistant
    let run_url = format!("https://api.openai.com/v1/threads/{}/runs", thread_id);
    let run_response = client
        .post(&run_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .header("OpenAI-Beta", "assistants=v2")
        .json(&json!({
            "assistant_id": assistant_id
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let run_id = run_response["id"].as_str().ok_or("Missing run ID")?;
    println!("Run ID: {}", run_id);

    // Step 4: Poll the run status until it's completed
    let check_run_url = format!("https://api.openai.com/v1/threads/{}/runs/{}", thread_id, run_id);
    
    loop {
        sleep(Duration::from_secs(2)).await; // Wait before checking again

        let status_response = client
            .get(&check_run_url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("OpenAI-Beta", "assistants=v2")
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let status = status_response["status"].as_str().unwrap_or("unknown");
        println!("Run Status: {}", status);

        if status == "completed" {
            break;
        } else if status == "failed" {
            return Err("Assistant run failed".into());
        }
    }

    // Step 5: Retrieve the assistant's response
    let messages_url = format!("https://api.openai.com/v1/threads/{}/messages", thread_id);
    let messages_response = client
        .get(&messages_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("OpenAI-Beta", "assistants=v2")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    // Check if messages exist and print them
    if let Some(messages) = messages_response["data"].as_array() {
        for msg in messages {
            if let Some(role) = msg["role"].as_str() {
                if let Some(content_array) = msg["content"].as_array() {
                    let content_texts: Vec<String> = content_array.iter()
                        .filter_map(|c| c["text"]["value"].as_str().map(String::from))
                        .collect();

                    let content = content_texts.join("\n");
                    println!("[{}]: {}", role, content);
                }
            }
        }
    } else {
        println!("No messages found.");
    }

    Ok(())
}

pub async fn get_thread_log(thread_id: &str, api_key: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = format!("https://api.openai.com/v1/threads/{}/messages", thread_id);

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("OpenAI-Beta", "assistants=v2")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("Thread Log: {:#?}", response);
    
    Ok(())
}
