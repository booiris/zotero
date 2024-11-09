import { invoke } from "@tauri-apps/api/core"

export const download_pdf = async (key: string): Promise<void> => {
    return await invoke("download_pdf", { key })
}
