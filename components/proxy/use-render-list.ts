import { getProxies } from "@/services/api";
import useSWR from "swr";
import { HeadState } from "./use-head-state";

export interface IRenderItem {
  // 组 ｜ head ｜ item ｜ empty | item col
  type: 0 | 1 | 2 | 3 | 4;
  key: string;
  group: IProxyGroupItem;
  proxy?: IProxyItem;
  col?: number;
  proxyCol?: IProxyItem[];
  headState?: HeadState;
}

export const useRenderList = (mode: string) => {
  const { data: proxiesData, mutate: mutateProxies } = useSWR("getProxies", getProxies, { refreshInterval: 45000 });

  // const renderList: IRenderItem[] = useMemo(() => {
  //   if (!proxiesData) return []

  //   const useRule = mode === "rule" || mode === "script"

  // }, [])

  return {
    // renderList,
    onProxies: mutateProxies,
    // onHeadState: setHeadState,
  }
}