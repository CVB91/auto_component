use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};

//Call large language model

pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    // Extract API key information
    let api_key: String = env::var("OPENAI_API_KEY").expect("API key not found");
    let api_org: String = env::var("OPENAI_API_ORG").expect("OPEN_AI_ORG key not found");

    // confirm endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // create headers

    let mut headers = HeaderMap::new();

    // Create api key header
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    //Create OPEN_AI_ORG header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create client

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Create chat completion

    let chat_completion = ChatCompletion {
        model: "gpt-4".to_string(),
        messages,
        temperature: 0.1,
    };

    // Troubleshooting code to check if the chat completion is being created correctly

    // let response_raw= client
    //   .post(url)
    //   .json(&chat_completion)
    //   .send()
    //   .await
    //   .unwrap();

    // dbg!(response_raw.text().await.unwrap());

    // Extract API Response

    let response: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // send response
    Ok(response.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_gpt() {
        let message = vec![Message {
            role: "user".to_string(),
            content: "Hello, this is a test. Give me short response.".to_string(),
        }];

        let response = call_gpt(message).await;

        if let Ok(response_string) = response {
            dbg!(response_string);
            assert!(true)
        } else {
            assert!(false)
        }
    }
}
