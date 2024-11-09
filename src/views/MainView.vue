<template>
    <div class="page">
        <n-spin class="loading-spin" size="large" :show="loading">
            <template #description>
                <span class="loading-text">loading...</span>
            </template>

            <div v-if="!loading" class="main-content">
                <n-layout has-sider>
                    <side-menu @update-collapsed="handleCollapsed" />
                    <n-layout-content class="content-area" :style="contentStyle">
                        <router-view />
                    </n-layout-content>
                </n-layout>
            </div>
        </n-spin>
    </div>
</template>

<script lang="ts" setup>
import { refresh } from '@/api/refresh';
import { useMessage } from 'naive-ui';
import { ref, onMounted, h, computed } from 'vue'

const loading = ref(true)
const message = useMessage()
const isCollapsed = ref(false)

const contentStyle = computed(() => ({
    marginLeft: isCollapsed.value ? '64px' : '240px'
}))

const handleCollapsed = (collapsed: boolean) => {
    isCollapsed.value = collapsed
}

onMounted(async () => {
    await refresh().catch((e) => {
        message.error(() => h('div', [
            h('div', 'refresh failed'),
            h('div', '[error]: ' + e.toString())
        ]))
    }).finally(() => {
        loading.value = false
    })
})
</script>

<style scoped>
.page {
    width: 100%;
    height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
}

.loading-spin {
    width: 100%;
    height: 100%;
}

.loading-text {
    font-size: 14px;
    color: #666;
    margin-top: 8px;
}

.main-content {
    width: 100%;
    height: 100%;
}

.content-area {
    transition: margin-left 0.2s;
    height: 100vh;
    overflow-y: auto;
}

:deep(.n-layout-scroll-container) {
    overflow: visible;
}

:deep(.n-spin-body) {
    width: 100%;
    height: 100%;
}
</style>