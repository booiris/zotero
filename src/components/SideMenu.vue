<template>
    <div class="side-menu">
        <n-layout-sider collapse-mode="width" :collapsed-width="64" :width="240" :collapsed="collapsed">
            <div class="menu-container">
                <!-- 基础菜单项 -->
                <div class="menu-item disabled" :class="{ 'menu-collapsed': collapsed }">
                    <div class="icon-wrapper">
                        <n-icon size="24">
                            <BookOutline />
                        </n-icon>
                    </div>
                    <span class="menu-label" :class="{ 'label-collapsed': collapsed }">Zotero Library</span>
                </div>

                <div v-show="!collapsed" class="menu-divider"></div>

                <!-- All Items -->
                <div class="menu-item" :class="{ active: selectedKey === 'all-items', 'menu-collapsed': collapsed }"
                    @click="handleSelect('all-items')">
                    <div class="icon-wrapper">
                        <n-icon size="24">
                            <FileTrayFullOutline />
                        </n-icon>
                    </div>
                    <span v-show="!collapsed" class="menu-label">All Items</span>
                    <span v-show="!collapsed" class="menu-badge">{{ allItemsCount }}</span>
                </div>

                <!-- 动态生成的集合菜单 -->
                <div v-for="item in menuItems" :key="item.key" class="menu-group">
                    <div class="menu-item" :class="{ active: selectedKey === item.key, 'menu-collapsed': collapsed }"
                        @click="handleSelect(item.key)">
                        <div class="icon-wrapper">
                            <n-icon size="24">
                                <FolderOutline />
                            </n-icon>
                        </div>
                        <span v-show="!collapsed" class="menu-label">{{ item.label }}</span>
                        <n-icon v-if="item.children?.length && !collapsed" class="expand-icon"
                            :class="{ expanded: isExpanded(item.key) }" @click.stop="toggleExpand(item.key)">
                            <ChevronDownOutline />
                        </n-icon>
                    </div>

                    <!-- 子菜单 -->
                    <div v-if="item.children?.length && isExpanded(item.key) && !collapsed" class="submenu">
                        <div v-for="child in item.children" :key="child.key" class="menu-item"
                            :class="{ active: selectedKey === child.key }" @click="handleSelect(child.key)">
                            <div class="icon-wrapper">
                                <n-icon size="20">
                                    <FolderOutline />
                                </n-icon>
                            </div>
                            <span class="menu-label">{{ child.label }}</span>
                            <n-icon v-if="child.children?.length" class="expand-icon"
                                :class="{ expanded: isExpanded(child.key) }" @click.stop="toggleExpand(child.key)">
                                <ChevronDownOutline />
                            </n-icon>
                        </div>
                    </div>
                </div>

                <!-- Unfiled Items -->
                <div class="menu-item" :class="{ active: selectedKey === 'unfiled-items', 'menu-collapsed': collapsed }"
                    @click="handleSelect('unfiled-items')">
                    <div class="icon-wrapper">
                        <n-icon size="24">
                            <DocumentsOutline />
                        </n-icon>
                    </div>
                    <span v-show="!collapsed" class="menu-label">Unfiled Items</span>
                    <span v-show="!collapsed" class="menu-badge">{{ unfiledItemsCount }}</span>
                </div>
            </div>
        </n-layout-sider>

        <n-switch v-model:value="collapsed" size="large" class="collapse-switch">
            <template #checked-icon>
                <n-icon>
                    <ArrowForwardOutline />
                </n-icon>
            </template>
            <template #unchecked-icon>
                <n-icon>
                    <ArrowBackOutline />
                </n-icon>
            </template>
        </n-switch>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, h } from 'vue'
import { NIcon, useMessage } from 'naive-ui'
import {
    BookOutline,
    FileTrayFullOutline,
    DocumentsOutline,
    FolderOutline,
    ArrowBackOutline,
    ArrowForwardOutline,
    ChevronDownOutline,
} from '@vicons/ionicons5'
import { Collection, get_collections } from '@/api/get_collections'
import { useRouter } from 'vue-router'

const collapsed = ref(false)
const expandedKeys = ref<Set<string>>(new Set())
const selectedKey = ref<string | null>(null)
const menuItems = ref<any[]>([])
const allItemsCount = ref(0)
const unfiledItemsCount = ref(0)

watch(collapsed, (newValue) => {
    if (newValue) {
        expandedKeys.value.clear()
    }
    emit('update-collapsed', newValue)
})

const isExpanded = (key: string) => expandedKeys.value.has(key)

const toggleExpand = (key: string) => {
    if (expandedKeys.value.has(key)) {
        expandedKeys.value.delete(key)
    } else {
        expandedKeys.value.add(key)
    }
}

const router = useRouter()

const handleSelect = (key: string) => {
    selectedKey.value = key
    router.push(`/main/collection/${key}`)
}

type MenuItem = {
    key: string
    label: string
    children?: MenuItem[]
}

const processCollections = (collections: Collection[]): MenuItem[] => {
    if (!collections) return []

    return collections.map(collection => ({
        key: collection.key,
        label: collection.name,
        children: processCollections(collection.children)
    }))
}

const message = useMessage()

onMounted(async () => {
    try {
        const collections = await get_collections()
        console.log("load collections success")
        console.log(JSON.stringify(collections))
        menuItems.value = processCollections(collections[0])
        allItemsCount.value = collections[1]
        unfiledItemsCount.value = collections[2]
    } catch (e) {
        console.error(e)
        message.error(() => h('div', [
            h('div', 'get collections failed'),
            h('div', '[error]: ' + e)
        ]))
    }
})

const emit = defineEmits(['update-collapsed'])
</script>

<style scoped>
.side-menu {
    height: 100vh;
    position: fixed;
    left: 0;
    top: 0;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    border-right: 1px solid #f0f0f0;
}

.menu-container {
    padding: 4px;
    overflow: hidden;
}

.menu-item {
    display: flex;
    align-items: center;
    padding: 10px 12px;
    margin: 2px 0;
    cursor: pointer;
    border-radius: 4px;
    transition: all 0.2s;
    color: #333;
    overflow: hidden;
}

.menu-item:not(.disabled):hover {
    background-color: rgba(0, 0, 0, 0.06);
}

.menu-item.active {
    background-color: var(--primary-color, #2db96e) !important;
    color: white;
}

.menu-item.disabled {
    cursor: not-allowed;
    opacity: 0.5;
}

.menu-label {
    margin-left: 12px;
    flex: 1;
    white-space: nowrap;
    transition: all 0.2s ease-in-out;
    overflow: hidden;
}

.menu-badge {
    background-color: #686868;
    color: white;
    padding: 4px 8px;
    border-radius: 12px;
    font-size: 12px;
    line-height: 1;
    transition: all 0.2s ease-in-out;
}

.menu-divider {
    height: 1px;
    background-color: #eee;
    margin: 8px 0;
}

.expand-icon {
    transition: all 0.2s ease-in-out;
    font-size: 16px;
    flex-shrink: 0;
}

.expand-icon.expanded {
    transform: rotate(180deg);
}

.submenu {
    margin-left: 24px;
}

.submenu .menu-item {
    font-size: 0.95em;
}

.collapse-switch {
    margin: 16px;
    align-self: flex-end;
    margin-top: auto;
}

:deep(.n-icon) {
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 24px;
}

.menu-collapsed {
    padding: 10px 0;
    justify-content: center;
    width: 64px;
}

.menu-collapsed .n-icon {
    margin: 0 auto;
    padding: 0;
}

.menu-collapsed .expand-icon {
    display: none;
}

.menu-collapsed .menu-label,
.menu-collapsed .menu-badge {
    opacity: 0;
    width: 0;
    margin: 0;
}

.label-collapsed {
    width: 0;
    margin: 0;
    opacity: 0;
    padding: 0;
}

.icon-wrapper {
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
}

.menu-collapsed .icon-wrapper {
    margin: 0 auto;
}
</style>