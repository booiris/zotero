<template>
    <div class="login-container">

        <div class="hero-section">
            <n-image src="/book.svg" alt="Logo" class="logo" preview-disabled />
        </div>

        <div class="form-section">
            <n-input v-model:value="apiKey" type="password" placeholder="Input your Zotero API Key..."
                :input-props="{ autocomplete: 'off' }">
                <template #prefix>
                    <n-icon>
                        <KeyOutline />
                    </n-icon>
                </template>
            </n-input>


            <n-button type="primary" block circle color="black" :loading="loading" @click="handleLogin"
                class="login-button">
                login
            </n-button>


            <div class="hint-text">
                <n-text depth="3">
                    获取 API Key 请访问 Zotero 官网的个人设置页面
                </n-text>
            </div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { useMessage } from 'naive-ui'
import { KeyOutline } from '@vicons/ionicons5'
import { login } from '@/api/login'
import router from '@/router'

const message = useMessage()
const apiKey = ref(import.meta.env.VITE_API_KEY)
const loading = ref(false)

const handleLogin = async () => {
    if (!apiKey.value) {
        message.warning('please input your Zotero API Key')
        return
    }

    loading.value = true
    try {
        await login(apiKey.value)
        message.success('login success', { duration: 800 })
        await new Promise(resolve => setTimeout(resolve, 300))
        router.push({ name: 'main' })
    } catch (error) {
        message.error('login failed\n' + error)
    } finally {
        loading.value = false
    }
}
</script>

<style scoped>
.login-container {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    background-color: #ffffff;
}

.hero-section {
    flex: 1;
    display: flex;
    justify-content: center;
    align-items: center;
    padding-top: 60px;
}

.logo {
    width: 180px;
    height: 180px;
}

.form-section {
    flex: 1;
    padding: 32px 16px;
    max-width: 400px;
    margin: 0 auto;
    width: 100%;

    box-sizing: border-box
}

.login-button {
    margin-top: 24px;
}

.hint-text {
    margin-top: 24px;
    text-align: center;
}
</style>