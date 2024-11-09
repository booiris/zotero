import { invoke } from "@tauri-apps/api/core"

export type Collection = {
    name: string
    key: string
    children: Collection[]
}

export const get_collections = async (): Promise<[Collection[], number, number]> => {
    return await invoke("get_collections")
}
