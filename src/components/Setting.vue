<template>
    <div class="setting-container flex flex-col">
        <Tabs v-model:activeKey="activeKey" type="card">
            <TabPane key="0" tab="通用">
                <Input v-model:value="svnPath" placeholder="svn地址" readonly>
                <template #prefix>
                    <i i-logos-subversion />
                </template>
                <template #suffix>
                    <Button @click="getVersionPath('svn')" size="small">修改</Button>
                </template>
                </Input>
                <Input class="mt-2" v-model:value="gitPath" placeholder="git地址" readonly>
                <template #prefix>
                    <i i-logos-git-icon />
                </template>
                <template #suffix>
                    <Button @click="getVersionPath('git')" size="small">修改</Button>
                </template>
                </Input>
                <InputPassword class="mt-2" v-model:value="apiKey" placeholder="API Key">
                    <template #prefix>
                        <i i-flat-color-icons-key />
                    </template>
                </InputPassword>
                <InputPassword class="mt-2" v-model:value="secretKey" placeholder="Secret Key">
                    <template #prefix>
                        <i i-flat-color-icons-data-encryption />
                    </template>
                </InputPassword>
                <Button @click="saveConfig" mt-2>
                    <template #icon><i i-bi-save2 class="mr-2 " /></template>
                    保存</Button>
            </TabPane>
            <TabPane key="1" tab="仓库">
                <button text-violet b-violet flex items-center justify-center b-1 b-rounded p-1 class="hover:bg-violet/10"
                    @click="choseRepository">
                    <i i-bi-window-plus />
                    选择本地仓库
                </button>

                <ul mt-2 p-1 b-1 b-rounded shadow-gray shadow>
                    <li class="flex   items-center bg-pink-100/10 hover:bg-blue-100/50 select-none cursor-default siblings-mt-1 b-1 b-lightBlue/50"
                        v-for="item in store.pathList" :key="item">
                        <i i-bi-hash text-xl></i>
                        <span text-bluegray underline-violet underline font-500>{{ item }}</span>
                        <i i-line-md-close-circle ml-a cursor-pointer @click="onDeletePath(item)" />
                    </li>
                </ul>
            </TabPane>
        </Tabs>
    </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { Input, InputPassword, Button, message, Tabs, TabPane } from 'ant-design-vue'
import { open } from '@tauri-apps/api/dialog';
import { BaseDirectory, readTextFile, writeTextFile } from '@tauri-apps/api/fs';
import { getPathList } from '@/utils';
import { useStore, useStoreSetItem } from '@/store';

const store = useStore()
const activeKey = ref('0')
const svnPath = ref('')
const gitPath = ref('')
const apiKey = ref('')
const secretKey = ref('')

/**
 * 获取版本控制工具地址
 * @param type 
 */
async function getVersionPath(type: string) {
    const selected = await open({
        multiple: false,
        directory: false,
    });
    if (selected !== null) {
        if (type === 'svn') {
            svnPath.value = selected as string
        } else if (type === 'git') {
            gitPath.value = selected as string
        }
        saveConfig()
    }
}

// 1. 获取本地配置
readConfig()
/**
 * 读取本地配置文件
 */
function readConfig() {
    readTextFile('conf/settings.conf', {
        dir: BaseDirectory.AppConfig,
    }).then((res) => {
        const jsonObj = JSON.parse(res)
        svnPath.value = jsonObj.svnPath
        gitPath.value = jsonObj.gitPath
        apiKey.value = jsonObj.apiKey
        secretKey.value = jsonObj.secretKey
    }).catch(() => {
        message.error('获取配置失败')
    })
}

/**
 * 保存配置
 */
function saveConfig() {
    const jsonString = JSON.stringify({
        svnPath: svnPath.value,
        gitPath: gitPath.value,
        apiKey: apiKey.value,
        secretKey: secretKey.value,
    })
    writeTextFile('conf/settings.conf', jsonString, {
        dir: BaseDirectory.AppConfig,
        append: false,
    }).then(() => {
        message.success('保存成功')
    }).catch(() => {
        message.error('保存失败')
    })
}


/**
 * 选择版本仓库地址
 */
const choseRepository = async () => {
    const selected = await open({
        multiple: false,
        directory: true,
    });
    if (selected === null) {
        // user cancelled the selection
        // pathFile.value = '';
        return;
    } else {
        if (store.value.pathList.includes(selected as string)) return;
       await writeTextFile('conf/app.conf', selected + '\n', {
            dir: BaseDirectory.AppConfig,
            append: true,
        });
        const pathList = await getPathList();
        useStoreSetItem('pathList', pathList)
    }
};

const onDeletePath = (p: string) => {
    const temp = store.value.pathList.filter((e) => e !== p).map(item => item);
    useStoreSetItem('pathList', temp)
    writeTextFile('conf/app.conf', temp.join("\n")+ "\n", {
        dir: BaseDirectory.AppConfig,
    })
}
</script>