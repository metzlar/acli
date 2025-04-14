use serde::{Serialize, Deserialize};
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use std::env;

const URI: &str = "https://api.openai.com/v1/responses";
const MODEL: &str = "gpt-4o";

#[derive(Serialize, Deserialize, Debug)]
struct OAIRequest {
    model: String,
    input: String,
}

#[derive(Debug, Deserialize)]
struct OAIResponse {
    // id: String, //"resp_67fc248419bc8192bfb0de105b1bdc340d6b697861d57ecb",
    // object: String, //"response",
    // created_at: i64, //1744577668,
    // status: String, //"completed",
    //"error": null,
    //"incomplete_details": null,
    //"instructions": null,
    //"max_output_tokens": null,
    // model: String, //"gpt-4o-2024-08-06",
    output: Vec<OAIOutput>,
    //"parallel_tool_calls": true,
    //"previous_response_id": null,
    // "reasoning": {
    //     "effort": null,
    //     "generate_summary": null
    // },
    // "store": true,
    // "temperature": 1.0,
    // "text": {
    //     "format": {
    //     "type": "text"
    //     }
    // },
    // "tool_choice": "auto",
    // "tools": [],
    // "top_p": 1.0,
    // "truncation": "disabled",
    // "usage": {
    //     "input_tokens": 10,
    //     "input_tokens_details": {
    //     "cached_tokens": 0
    //     },
    //     "output_tokens": 314,
    //     "output_tokens_details": {
    //     "reasoning_tokens": 0
    //     },
    //     "total_tokens": 324
    // },
    // "user": null,
    // "metadata": {}
}

#[derive(Debug, Deserialize)]
struct OAIOutput {
    // id: String,
    // r#type: String,
    // status: String,
    // role: String,
    content: Vec<OAIContent>,
}

#[derive(Debug, Deserialize)]
struct OAIContent {
    // r#type: String,
    //"annotations": [],
    text: String,
}

async fn query(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let oai_token = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", oai_token).parse().unwrap(),
    );


    let req = OAIRequest {
        model: String::from(MODEL),
        input: text.to_string(),
    };

    let res = client
        .post(URI)
        .headers(headers)
        .json(&req)
        .send()
        .await?
        .text()
        .await?;

    println!("Output: {:?}", res);

    let res_json:OAIResponse = serde_json::from_str(res.as_str())?;

    let message = res_json
        .output
        .last()
        .ok_or("Missing `output`")?
        .content.last()
        .ok_or("Missing content")?
        .text
        .clone();

    Ok(message)
}

pub async fn r#do(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let command = format!("Construct a shell command that: {}", text);
    query(&command).await
}

pub async fn explain(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let command = format!("Explain this shell command: {}", text);
    query(&command).await
}