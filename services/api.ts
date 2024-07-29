import axios, { AxiosInstance } from "axios";
import { getClashInfo } from "./cmds";

let axiosIns: AxiosInstance = null!;

export const getAxios = async (force: boolean = false): Promise<AxiosInstance> => {
    if (axiosIns && !force) return axiosIns

    let server = "";
    let secret = "";

    try {
        const info = await getClashInfo();
        console.log("====", info)

        if (info?.server) {
            server = info.server;
            if (/^\d+$/.test(server)) server = `127.0.0.1:${server}`
        }
        if (info?.secret) secret = info!.secret;

    } catch (err) {
    }

    axiosIns = axios.create({
        baseURL: `http://${server}`,
        headers: secret ? { "Authorization": `Bearer ${secret}` } : {},
        timeout: 15000,
    })

    axiosIns.interceptors.response.use((r) => r.data)
    return axiosIns
}


export const getClashConfig = async (): Promise<IConfigData> => {
    const instance = await getAxios()
    return instance?.get("/configs") as Promise<IConfigData>
}

export const updateConfigs = async (config: Partial<IConfigData>) => {
    const instance = await getAxios()
    return instance?.patch("/configs", config)
}

export const getProxies = async () => {
    const [proxyRecord, providerRecord] = await Promise.all([
        getProxiesInner(),
        getProxyProviders(),
    ])
}

export const getProxiesInner = async () => {
    const instance = await getAxios();
    const response = await instance.get<any, any>("/proxies");
    return (response?.proxies || {}) as Record<string, IProxyItem>
}

export const getProxyProviders = async () => {
    const instance = await getAxios();
    const response = await instance.get<any, any>("/providers/proxies");

    const providers = (response.providers || {}) as Record<string, IProxyProviderItem>;
}