use crate::read_file::read_conf;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use tauri::Window;
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
    created: u64,
    sentence_id: u64,
    is_end: bool,
    is_truncated: bool,
    result: String,
    // ban_round: i32,
    need_clear_history: bool,
    usage: Usage,
}

pub async fn get_ai_text(window: Window, c: String) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let conf = read_conf();
    let path: String = format!("https://aip.baidubce.com/oauth/2.0/token?client_id={}&client_secret={}&grant_type=client_credentials",conf.api_key,conf.secret_key);
    let resp = client.post(path).send().await?;
    let value = resp.text().await?;
    let obj = serde_json::from_str::<Auth>(&value)?;
    get_ai(window, obj.access_token.as_str(), c).await?;

    Ok(())
}

async fn get_ai<'a>(window: Window, token: &'a str, content: String) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let url = format!("https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/chatglm2_6b_32k?access_token={}",token);
    let mut resp = client
        .post(url)
        .header("conetnt-type", "application/json")
        .body(
            json!({"messages" : [
             {"role":"user","content": content}
            ],"stream": true})
            .to_string(),
        )
        .send()
        .await?
        .bytes_stream();

    while let Some(item) = resp.next().await {
        let text = String::from_utf8(item.unwrap().into()).unwrap();
        let text = text.get(6..).unwrap();

        match serde_json::from_str::<AiResponse>(text) {
            Ok(s) => {
                // println!("{:?}", s);
                let _ = window.emit("msg-stream", &s);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
    Ok(())
}
