import { invoke } from "@tauri-apps/api/core"

export const refresh = async () => {
    await invoke("refresh")
}
