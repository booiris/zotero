import { invoke } from "@tauri-apps/api/core"

export const is_login = async (): Promise<boolean> => {
    return await invoke("is_login")
}
