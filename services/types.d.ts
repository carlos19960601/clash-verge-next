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