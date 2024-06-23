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