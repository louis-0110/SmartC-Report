<script setup lang="ts">
import OpenFiles from '@/components/OpenFiles.vue'
import zhCN from 'ant-design-vue/es/locale/zh_CN';
import { ConfigProvider, Modal } from 'ant-design-vue'
import dayjs from 'dayjs';
import 'dayjs/locale/zh-cn';
import { theme } from 'ant-design-vue';
import { computed, ref } from 'vue';
import Setting from './components/Setting.vue';
import { getPathList } from './utils';
import { BaseDirectory, createDir, writeTextFile } from '@tauri-apps/api/fs';
import { useStoreSetItem } from './store';

dayjs.locale('zh-cn');

let isDarkMode = ref(window.matchMedia('(prefers-color-scheme: dark)').matches)
let algorithm = computed(() => isDarkMode.value ? theme.darkAlgorithm : theme.defaultAlgorithm)

const openSetting = ref(false)
const onOpend = () => { openSetting.value = !openSetting.value }


  // 初始化配置文件
  ; (async () => {
    try {
      const pArr = await getPathList();
      useStoreSetItem('pathList', pArr);
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
</script>

<template>
  <ConfigProvider :locale="zhCN" :theme="{
    algorithm
  }">
    <div class="container">
      <OpenFiles />
      <span class="absolute right-4 top-1 text-20px b-black-500 b-2 b-rounded hover:b-black/30 cursor-pointer" >
        <i i-line-md-close-to-menu-transition @click="onOpend" />
      </span>


      <Modal class="bg-white pb-0" v-model:open="openSetting" width="100%" wrap-class-name="full-modal" :closable="false"
        :footer="null" @ok="onOpend">
        <Setting />
      </Modal>
    </div>
  </ConfigProvider>
</template>

<style scoped>
.container {
  margin: auto;
  height: 100%;
}

/* .container:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.container:hover {
  filter: drop-shadow(0 0 2em #249b73);
} */
</style>
