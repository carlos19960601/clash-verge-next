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


  const generateItem = (name: string) => {

    return {
      name,
      type: "unknown",
      udp: false,
      xudp: false,
      tfo: false,
      history: []
    }
  }


  const { GLOBAL: global, DIRECT: direct, REJECT: rejext } = proxyRecord;


  let groups: IProxyGroupItem[] = Object.values(proxyRecord).reduce<IProxyGroupItem[]>((acc, each) => {
    if (each.name !== "GLOBAL" && each.all) {
      acc.push({
        ...each,
        all: each.all!.map((item) => generateItem(item))
      })
    }
    return acc;
  }, [])

  if (global?.all) {
    let globalGroups: IProxyGroupItem[] = global.all.reduce<IProxyGroupItem[]>((acc, name) => {
      return acc
    }, [])

    let globalNames = new Set(globalGroups.map((each) => each.name));
    groups = groups.filter((group) => {
      return !globalNames.has(group.name);
    }).concat(globalGroups)
  }
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

  return Object.fromEntries(Object.entries(providers).filter(([key, item]) => {
    const type = item.vehicleType.toLowerCase()
    return type === "http" || type === "file"
  }))
}