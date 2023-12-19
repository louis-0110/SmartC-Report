import { invoke } from '@tauri-apps/api/tauri';

export async function savePath(
  p: string[],
  d: [string, string]
): Promise<string[]> {
  return await invoke('save_path', { path: p, currentDate: d });
}

export async function getAiContent(value: string) {
  await invoke('get_ai_content', { c: value });
}
