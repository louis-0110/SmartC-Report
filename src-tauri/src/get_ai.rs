use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json::json;

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
    let resp = client.post("https://aip.baidubce.com/oauth/2.0/token?client_id=Wq2mGzgQX9Q13f2tg1gZtQyy&client_secret=zuxcm9DtemprEg8rf2FzTgLyNG2KZSDf&grant_type=client_credentials")
        .send().await?;
    let value = resp.text().await?;
    let obj = serde_json::from_str::<Auth>(&value)?;
    let text = get_ai(obj.access_token.as_str(), c).await?;
    let res = serde_json::from_str::<AiResponse>(&text)?;
    Ok(res.result)
}

async fn get_ai<'a>(token: &'a str, content: String) -> Result<String, Box<dyn Error>> {
    let template = format!(
        "请你根据我的工作产出为我生成一份日报。
    要求润色我的工作成果并为我制定明日工作计划。
    结果需要以列表的形式呈现。
    我的主要工作产出是：{content}"
    );
    let client = reqwest::Client::new();
    let url = format!("https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/chatglm2_6b_32k?access_token={}",token);
    let resp = client
        .post(url)
        .header("conetnt-type", "application/json")
        .body(
            json!({"messages" : [
             {"role":"user","content": template}
            ]})
            .to_string(),
        )
        .send()
        .await?;
    let value = resp.text().await?;
    Ok(value)
}
