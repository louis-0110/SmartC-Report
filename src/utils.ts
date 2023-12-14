import { BaseDirectory, readTextFile } from '@tauri-apps/api/fs';

export async function getPathList() {
  const contents = await readTextFile('conf/app.conf', {
    dir: BaseDirectory.AppConfig,
  });
  return contents.split('\n').filter((item) => item.trim());
}
