import { getClient, ResponseType } from '@tauri-apps/api/http';

export async function get_token(): Promise<any> {
  const client = await getClient();
  const response = await client.post(
    `https://aip.baidubce.com/oauth/2.0/token?grant_type=client_credentials&client_id=Wq2mGzgQX9Q13f2tg1gZtQyy&client_secret=zuxcm9DtemprEg8rf2FzTgLyNG2KZSDf`,
    undefined,
    {
      // in this case the server returns a simple string
      responseType: ResponseType.JSON,
      headers: {
        'Content-Type': 'application/json',
      },
    }
  );
  return response;
}

export async function get_text(
  token: string,
  content: string
): Promise<string> {
  const res = await fetch(
    'https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/chatglm2_6b_32k?access_token=' +
      token,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json; charset=UTF-8',
      },
      body: `"messages": [
        {"role":"user","content":${content}}
       ]`,
    }
  );

  return await res.json();
}
