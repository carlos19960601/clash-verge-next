import { invoke } from "@tauri-apps/api/tauri";

export async function getClashInfo() {
    return invoke<IClashInfo | null>("get_clash_info")
}