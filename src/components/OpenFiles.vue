<template>
  <div class="flex items-center">
    <button
      class="hover:bg-violet/10"
      b-rounded
      text-violet
      b-violet
      b-1
      p-1
      mr-2
      @click="get_ai_log"
    >
      获取日志
      <i v-if="isLoadingLog" i-line-md-loading-twotone-loop />
    </button>
    <button
      class="hover:bg-violet/10"
      b-rounded
      text-violet
      b-violet
      b-1
      p-1
      mr-2
      @click="create_report"
    >
      生成日报
      <i v-if="isLoadingReprot" i-line-md-loading-twotone-loop />
    </button>
    <RangePicker
      v-model:value="dateTime"
      valueFormat="YYYY-MM-DD"
      format="YYYY-MM-DD"
      size="small"
    />
  </div>
  <Textarea
    class="my-1 b-1 p-2"
    v-model:value="inputValue"
    auto-size
  ></Textarea>

  <div
    class="items-center w-full b-1 p-2 h-480px overflow-y-auto shadow-sm break-all relative"
  >
    <button
      class="hover:bg-violet/10"
      b-rounded
      text-violet
      b-violet
      text-12px
      px-2
      b-1
      absolute
      right-4
      top-4
      @click="ai_result = ''"
    >
      清空
    </button>
    <p v-html="ai_result"></p>
  </div>
</template>

<script setup lang="ts">
import dayjs from 'dayjs';
import { ref, onBeforeUnmount } from 'vue';
import { getAiContent, savePath } from '@/emits/index';
import { listen } from '@tauri-apps/api/event';
import { RangePicker } from 'ant-design-vue';
import { Textarea } from 'ant-design-vue';
import { useStore } from '@/store';

const logs = ref<string[]>([]);
const ai_result = ref('');
const today = dayjs().format('YYYY-MM-DD');
const dateTime = ref<[string, string]>([today, today]);

const LOG_CUR_WORDS = '请你根据我的工作产出为我生成一份日报。要求润色我的工作成果并为我制定明日工作计划。结果需要以列表的形式呈现。我的主要工作产出是:';
const inputValue = ref("");
const listenEenvt = async () => {
  return await listen<{
    result: string;
  }>('msg-stream', (event) => {
    ai_result.value += event.payload?.result.replace(/\n/g, '<br/>');
  });
};
const unlisten = listenEenvt();

onBeforeUnmount(() => {
  unlisten.then((close) => close());
});

const isLoadingLog = ref(false);
const isLoadingReprot = ref(false);

async function get_ai_log() {
  isLoadingLog.value = true;
  let pathList = useStore().value.pathList;
  logs.value = await savePath(
    pathList.map((item) => item),
    dateTime.value
  );
  inputValue.value = LOG_CUR_WORDS + logs.value.reduce((pre, cur) => (pre += cur + '\n'), '');
  isLoadingLog.value = false;
}

async function create_report() {
  isLoadingReprot.value = true;
  await getAiContent(inputValue.value);
  isLoadingReprot.value = false;
}
</script>
