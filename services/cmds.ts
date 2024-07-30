import { invoke } from "@tauri-apps/api/tauri";

export async function getClashInfo() {
    return invoke<IClashInfo | null>("get_clash_info")
}

export async function getProfiles() {
    return invoke<IProfilesConfig>("get_profiles")
}

export async function importProfile(url: string) {
    return invoke<void>("import_profile", { url, option: { with_proxy: true } });
}

export async function getVergeConfig() {
    return invoke<IVergeConfig>("get_verge_config")
}

export async function patchVergeConfig(payload: IVergeConfig) {
    return invoke<void>("patch_verge_config", { payload })
}

