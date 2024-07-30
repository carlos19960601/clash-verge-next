interface IClashInfo {
    mixed_port?: number;
    socks_port?: number;
    server?: string;
    secret?: string;
}

interface IConfigData {
    port: number;
    mode: string;
    "mixed_port": number;
    "allow-lan": boolean;
}

type IProxyGroupItem = Omit<IProxyItem, "all"> & {
    all: IProxyItem[];
}

interface IProxyItem {
    name: string;
    type: string;
    udp: boolean;
    xudp: boolean;
    tfo: boolean;
    history: {
        time: string;
        delay: number;
    }[];
    all?: string[];
    now?: string;
    hidden?: boolean;
    icon?: string;
    provider?: string; // 记录是否来自provider
    fixed?: string; // 记录固定(优先)的节点
}



interface IProxyProviderItem {
    name: string;
    type: string;
    proxies: IProxyItem[];
    updatedAt: string;
    vehicleType: string;
    subscriptionInfo?: {
        Upload: number;
        Download: number;
        Total: number;
        Expire: number;
    };
}

interface IProfilesConfig {
    current?: string;
    chain?: string[];
    valid?: string[];
    items?: IProfileItem[];
}


interface IProfileItem {
    uid: string;
    type?: "local" | "remote" | "merge" | "script";
    name?: string;
    desc?: string;
    file?: string;
    url?: string;
    updated?: number;
    selected?: {
        name?: string;
        now?: string;
    }[];
    extra?: {
        upload: number;
        download: number;
        total: number;
        expire: number;
    };
    option?: IProfileOption;
    home?: string;
}


interface IVergeConfig {
    app_log_level?: "trace" | "debug" | "info" | "warn" | "error" | string;
    language?: string;
    tray_event?: "main_window" | "system_proxy" | "tun_mode" | string;
    env_type?: "bash" | "cmd" | "powershell" | string;
    startup_script?: string;
    start_page?: string;
    clash_core?: string;
    theme_mode?: "light" | "dark" | "system";
    traffic_graph?: boolean;
    enable_memory_usage?: boolean;
    enable_group_icon?: boolean;
    menu_icon?: "monochrome" | "colorful" | "disable";
    tray_icon?: "monochrome" | "colorful";
    common_tray_icon?: boolean;
    sysproxy_tray_icon?: boolean;
    tun_tray_icon?: boolean;
    enable_tun_mode?: boolean;
    enable_auto_launch?: boolean;
    enable_service_mode?: boolean;
    enable_silent_start?: boolean;
    enable_system_proxy?: boolean;
    proxy_auto_config?: boolean;
    pac_file_content?: string;
    enable_random_port?: boolean;
    verge_mixed_port?: number;
    verge_socks_port?: number;
    verge_redir_port?: number;
    verge_tproxy_port?: number;
    verge_port?: number;
    verge_redir_enabled?: boolean;
    verge_tproxy_enabled?: boolean;
    verge_socks_enabled?: boolean;
    verge_http_enabled?: boolean;
    enable_proxy_guard?: boolean;
    proxy_guard_duration?: number;
    system_proxy_bypass?: string;
    web_ui_list?: string[];
    hotkeys?: string[];
    theme_setting?: {
        primary_color?: string;
        secondary_color?: string;
        primary_text?: string;
        secondary_text?: string;
        info_color?: string;
        error_color?: string;
        warning_color?: string;
        success_color?: string;
        font_family?: string;
        css_injection?: string;
    };
    auto_close_connection?: boolean;
    auto_check_update?: boolean;
    default_latency_test?: string;
    default_latency_timeout?: number;
    enable_builtin_enhanced?: boolean;
    auto_log_clean?: 0 | 1 | 2 | 3;
    proxy_layout_column?: number;
    test_list?: IVergeTestItem[];
}