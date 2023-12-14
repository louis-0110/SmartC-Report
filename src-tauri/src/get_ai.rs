use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::read_file::read_conf;

#[derive(Serialize, Deserialize, Debug)]
struct Auth {
    refresh_token: String,
    expires_in: u64,
    session_secret: String,
    access_token: String,
    scope: String,
    session_key: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Usage {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}
#[derive(Serialize, Deserialize, Debug)]
struct AiResponse {
    id: String,
    object: String,
    result: String,
    created: u64,
    is_truncated: bool,
    need_clear_history: bool,
    usage: Usage,
}

#[tokio::main]
pub async fn get_ai_text(c: String) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let conf = read_conf();
    let path: String = format!("https://aip.baidubce.com/oauth/2.0/token?client_id={}&client_secret={}&grant_type=client_credentials",conf.api_key,conf.secret_key);
    let resp = client.post(path).send().await?;
    let value = resp.text().await?;
    let obj = serde_json::from_str::<Auth>(&value)?;
    let text = get_ai(obj.access_token.as_str(), c).await?;
    let res = serde_json::from_str::<AiResponse>(&text)?;
    Ok(res.result)
}

async fn get_ai<'a>(token: &'a str, content: String) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = format!("https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/chatglm2_6b_32k?access_token={}",token);
    let resp = client
        .post(url)
        .header("conetnt-type", "application/json")
        .body(
            json!({"messages" : [
             {"role":"user","content": content}
            ]})
            .to_string(),
        )
        .send()
        .await?;
    let value = resp.text().await?;
    Ok(value)
}
