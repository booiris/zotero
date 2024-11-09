import { invoke } from "@tauri-apps/api/core"

export type SimpleItem = {
    title: string
    key: string
}

export const get_items_by_collection = async (collection_key: string): Promise<SimpleItem[]> => {
    return await invoke("get_items_by_collection", { collection_key })
}
