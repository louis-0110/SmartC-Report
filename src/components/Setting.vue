<template>
    <div class="setting-container flex flex-col">
        <Input v-model:value="svnPath" placeholder="svn地址" readonly>
        <template #prefix>
            <i i-logos-subversion />
        </template>
        <template #suffix>
            <Button @click="getPath('svn')" size="small">修改</Button>
        </template>
        </Input>
        <Input class="mt-2" v-model:value="gitPath" placeholder="git地址" readonly>
        <template #prefix>
            <i i-logos-git-icon />
        </template>
        <template #suffix>
            <Button @click="getPath('git')" size="small">修改</Button>
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
    </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { Input,InputPassword, Button, message } from 'ant-design-vue'
import { open } from '@tauri-apps/api/dialog';
import { BaseDirectory, readTextFile, writeTextFile } from '@tauri-apps/api/fs';

const svnPath = ref('')
const gitPath = ref('')
const apiKey = ref('')
const secretKey = ref('')

async function getPath(type: string) {
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

readConfig()
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

function saveConfig() {
    // 保存配置
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
</script>