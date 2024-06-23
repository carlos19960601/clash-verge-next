import axios, { AxiosInstance } from "axios";
import { getClashInfo } from "./cmds";

let axiosIns: AxiosInstance = null!;

export const getAxios = async (force: boolean = false): Promise<AxiosInstance> => {
    if (axiosIns && !force) return axiosIns

    let server = "";
    let secret = "";

    try {
        const info = await getClashInfo();

        if (info?.server) {
            server = info.server;

            if (/^\d+$/.test(server)) server = `127.0.0.1:${server}`
        }
        if (info?.secret) secret = info!.secret;

    } catch { }

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