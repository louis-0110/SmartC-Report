import { VALUE_SPLIT } from 'ant-design-vue/es/vc-cascader/utils/commonUtil';
import { readonly, ref } from 'vue';

interface Store {
  svnPath: string;
  gitPath: string;
  apiKey: string;
  secretKey: string;
  pathList: string[];
}

type Key = keyof Store;

const store = ref<Store>({
  svnPath: '',
  gitPath: '',
  apiKey: '',
  secretKey: '',
  pathList: [],
});

/**
 * 获取store（readonly)
 * @returns
 */
function useStore() {
  return readonly(store);
}

/**
 * 重新赋值key
 * @param key
 * @param value
 */
function useStoreSetItem(key: Key, value: any) {
  store.value[key] = value;
}

/**
 * 重新设置store
 * @param value
 */
function useStoreSet(value: Store) {
  store.value = value;
}

export { useStore, useStoreSet, useStoreSetItem };
