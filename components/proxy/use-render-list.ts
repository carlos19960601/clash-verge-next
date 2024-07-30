import { useVerge } from "@/hooks/use-verge";
import { getProxies } from "@/services/api";
import { useMemo } from "react";
import useSWR from "swr";
import { HeadState } from "./use-head-state";
import { useWindowWidth } from "./use-window-width";

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
  const { verge } = useVerge();
  const { width } = useWindowWidth();


  const renderList: IRenderItem[] = useMemo(() => {
    if (!proxiesData) return []

    const useRule = mode === "rule" || mode === "script"

  }, [])

  return {
    renderList,
    onProxies: mutateProxies,
    onHeadState: setHeadState,
  }
}