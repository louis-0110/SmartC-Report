<script setup lang="ts">
import OpenFiles from '@/components/OpenFiles.vue'
import zhCN from 'ant-design-vue/es/locale/zh_CN';
import { ConfigProvider } from 'ant-design-vue'
import dayjs from 'dayjs';
import 'dayjs/locale/zh-cn';
import { theme } from 'ant-design-vue';
import { computed, ref } from 'vue';

dayjs.locale('zh-cn');

let isDarkMode = ref(window.matchMedia('(prefers-color-scheme: dark)').matches)
let algorithm = computed(() => isDarkMode.value ? theme.darkAlgorithm : theme.defaultAlgorithm)
</script>

<template>
  <ConfigProvider :locale="zhCN" :theme="{
    algorithm
  }">
    <div class="container">
      <OpenFiles />
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
