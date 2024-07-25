"use client";

import BasePage from "@/components/layout/base/base-page";
import ProxyGroups from "@/components/proxy/proxy-groups";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { getClashConfig, updateConfigs } from "@/services/api";
import { useTranslations } from "next-intl";
import useSWR from "swr";

export default function Home() {
  const modeList = ["rule", "global", "direct"];
  const t = useTranslations();

  const { data: clashConfig, mutate: mutateClash } = useSWR(
    "getClashConfig",
    getClashConfig
  );

  const curMode = clashConfig?.mode?.toLowerCase();

  const onChangeMode = async (mode: string) => {
    await updateConfigs({ mode });
    mutateClash();
  };

  return (
    <BasePage
      title={t("Proxy Groups")}
      header={
        <div className="flex">
          <div className="flex -space-x-px">
            {modeList.map((mode) => (
              <Button
                key={mode}
                onClick={() => onChangeMode(mode)}
                variant="outline"
                className={cn(
                  "first:rounded-l-md last:rounded-r-md rounded-none",
                  curMode === mode && "bg-blue-500"
                )}
              >
                {t(mode)}
              </Button>
            ))}
          </div>
        </div>
      }
    >
      <ProxyGroups mode={curMode!} />
    </BasePage>
  );
}
