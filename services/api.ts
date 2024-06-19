import axios, { AxiosInstance } from "axios";

let axiosIns: AxiosInstance = null!;

export const getAxios = async (force: boolean = false) => {
    if (axiosIns && !force) return axiosIns

    let server = "";
    let secret = "";

    try {
        const info = await getClashInfo();
    } catch { }

    axiosIns = axios.create({
        baseURL: `http://${server}`,
        headers: secret ? { "Authorization": `Bearer ${secret}` } : {},
        timeout: 15000,
    })
}


export const getClashConfig = async () => {
    const instance = await getAxios()

}