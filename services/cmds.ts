import { invoke } from "@tauri-apps/api/tauri";

export async function getClashInfo() {
    return invoke<IClashInfo | null>("get_clash_info")
}

export async function getProfiles() {
    return invoke<IProfilesConfig>("get_profiles")
}

export async function importProfiles(url: string) {
    return invoke<void>("import_profiles", { url, option: { with_proxy: true } });
}