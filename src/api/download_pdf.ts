import { Channel, invoke } from "@tauri-apps/api/core"

export const download_pdf = async (key: string, channel: Channel<number>): Promise<void> => {
    return await invoke("download_pdf", { key, downloaded_size: channel })
}
