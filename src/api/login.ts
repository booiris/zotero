import { invoke } from "@tauri-apps/api/core"

export const login = async (api_key: string) => {
    await invoke("login", { api_key })
}
