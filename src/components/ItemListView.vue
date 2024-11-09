<template>
    <div class="item-list-container">
        <n-list>
            <n-list-item v-for="item in items" :key="item.key">
                <div class="item-cell">
                    <n-icon size="20" class="file-icon">
                        <DocumentOutline />
                    </n-icon>
                    <n-ellipsis style="max-width: 240px">
                        {{ item.title }}
                    </n-ellipsis>
                    <n-button circle round :loading="loadingStates[item.key]" @click="downloadPdf(item)">
                        <template #icon>
                            <n-icon>
                                <DownloadOutline />
                            </n-icon>
                        </template>
                    </n-button>
                </div>
            </n-list-item>
        </n-list>
    </div>
</template>

<script setup lang="ts">
import { ref, watch, reactive } from 'vue'
import { NIcon, NList, NListItem, useMessage } from 'naive-ui'
import { DocumentOutline, DownloadOutline } from '@vicons/ionicons5'
import { get_items_by_collection, type SimpleItem } from '@/api/get_item_by_collection'
import { useRoute } from 'vue-router'
import { download_pdf } from '@/api/download_pdf'

const items = ref<SimpleItem[]>([])
const route = useRoute()
const message = useMessage()
const loadingStates = reactive<Record<string, boolean>>({})

watch(
    () => route.params.key,
    async (newKey) => {
        if (newKey) {
            try {
                items.value = await get_items_by_collection(newKey as string)
                items.value.forEach(item => {
                    loadingStates[item.key] = false
                })
            } catch (e) {
                console.error('get collections items failed: ', e)
                message.error('get collections items failed: ' + e)
            }
        }
    },
    { immediate: true }
)

const downloadPdf = async (item: SimpleItem) => {
    loadingStates[item.key] = true
    await download_pdf(item.key)
        .then(() => {
            message.success('download pdf ' + item.title + ' success')
        })
        .catch((e) => {
            message.error('download pdf failed: ' + e)
        })
        .finally(() => {
            loadingStates[item.key] = false
        })
}
</script>

<style scoped>
.item-cell {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
}

.file-icon {
    color: #666;
    flex-shrink: 0;
}

.item-title {
    flex: 1;
    min-width: 0;
    max-width: 200px;
}


:deep(.n-list-item) {
    padding: 12px 16px;
}

:deep(.n-list) {
    border-top: 1px solid #eee;
    border-bottom: 1px solid #eee;
}
</style>