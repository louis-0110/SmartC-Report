<template>
  <button text-violet b-violet flex items-center justify-center b-1 b-rounded p-1 class="hover:bg-violet/10"
    @click="choseRepository">
    <i i-bi-window-plus />
    选择本地仓库
  </button>

  <ul mt-2 p-1 b-1 b-rounded shadow-gray shadow>
    <li
      class="flex   items-center bg-pink-100/10 hover:bg-blue-100/50 select-none cursor-default siblings-mt-1 b-1 b-lightBlue/50"
      v-for="item in pathList" :key="item">
      <i i-bi-hash text-xl></i>
      <span text-bluegray underline-violet underline font-500>{{ item }}</span>
      <i i-line-md-close-circle ml-a cursor-pointer @click="onDeletePath(item)" />
    </li>
  </ul>

  <div class="flex items-center">
    <button class="hover:bg-violet/10" b-rounded text-violet b-violet b-1 p-1 mr-2 @click="get_ai_log">
      获取日报
      <i v-if="isLoading" i-line-md-loading-twotone-loop />
    </button>
    <RangePicker v-model:value="dateTime" valueFormat="YYYY-MM-DD" format='YYYY-MM-DD' size="small"
      @change="onConfirmDate" />
  </div>
  <ul my-1 b-1 p-2>
    <li v-for="(log, index) in logs" :key="index">
      <span>{{ index + 1 }}:</span>
      {{ log }}
    </li>
  </ul>

  <pre items-center b-1 p-2 shadow-sm>
    {{ ai_result }}
  </pre>
</template>

<script setup lang="ts">
import { open } from '@tauri-apps/api/dialog';
import { onMounted, ref } from 'vue';
import { getAiContent, savePath } from '@/emits/index';
import { BaseDirectory, createDir, readTextFile, writeTextFile } from '@tauri-apps/api/fs';
import { RangePicker } from 'ant-design-vue'
import { message } from 'ant-design-vue';

const pathFile = ref('');
const logs = ref<string[]>([]);
const ai_result = ref('');
const dateTime = ref<[string, string]>(['', ''])
const pathList = ref<string[]>([]);

const getPathList = async () => {
  const contents = await readTextFile('conf/app.conf', {
    dir: BaseDirectory.AppConfig,
  });
  return contents.split('\n').filter((item) => item.trim());
};

const onConfirmDate = () => {
  message.info('日期选择完成');
}
// 初始化配置文件
(async () => {
  try {
    pathList.value = await getPathList();
  } catch (_) {
    await createDir('conf', {
      dir: BaseDirectory.AppData,
      recursive: true,
    });

    await writeTextFile('conf/app.conf', '', {
      dir: BaseDirectory.AppConfig,
      append: true,
    });
  }
})();

onMounted(() => {
  getPathList();
});

const choseRepository = async () => {
  const selected = await open({
    multiple: false,
    directory: true,
  });
  if (selected === null) {
    // user cancelled the selection
    pathFile.value = '';
  } else {
    // user selected a single file
    pathFile.value = selected as string;
    if (pathList.value.includes(pathFile.value)) return;

    writeTextFile('conf/app.conf', pathFile.value + '\n', {
      dir: BaseDirectory.AppConfig,
      append: true,
    });
    pathList.value = await getPathList();
  }
};

const isLoading = ref(false);
async function get_ai_log() {
  isLoading.value = true;
  logs.value = await savePath(pathList.value, `{${dateTime.value[0]}}:{${dateTime.value[1]}}`);
  const res = await getAiContent(logs.value);
  ai_result.value = res;
  console.log(res);
  isLoading.value = false;
}

const onDeletePath = (p: string) => {
  console.log(p)
  pathList.value = pathList.value.filter((e) => e !== p);
  writeTextFile('conf/app.conf', pathList.value.join('\n'), {
    dir: BaseDirectory.AppConfig,
  })
}
</script>
