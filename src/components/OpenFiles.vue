<template>
  <div class="flex items-center">
    <button class="hover:bg-violet/10" b-rounded text-violet b-violet b-1 p-1 mr-2 @click="get_ai_log">
      获取日志
      <i v-if="isLoadingLog" i-line-md-loading-twotone-loop />
    </button>
    <button class="hover:bg-violet/10" b-rounded text-violet b-violet b-1 p-1 mr-2 @click="create_report">
      生成日报
      <i v-if="isLoadingReprot" i-line-md-loading-twotone-loop />
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

import { inject, ref } from 'vue';
import { getAiContent, savePath } from '@/emits/index';

import { RangePicker } from 'ant-design-vue'
import { message } from 'ant-design-vue';

const logs = ref<string[]>([]);
const ai_result = ref('');
const dateTime = ref<[string, string]>(['', ''])

const pathList = inject('pathList', [])

const onConfirmDate = () => {
  message.info('日期选择完成');
}


const isLoadingLog = ref(false);
const isLoadingReprot = ref(false);

async function get_ai_log() {
  isLoadingLog.value = true;
  logs.value = await savePath(pathList, dateTime.value);
  isLoadingLog.value = false;
}

async function create_report() {
  isLoadingReprot.value = true;
  const res = await getAiContent(logs.value);
  ai_result.value = res;
  isLoadingReprot.value = false;
}


</script>
