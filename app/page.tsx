"use client";

import BasePage from "@/components/layout/base/base-page";
import ProxyGroups from "@/components/proxy/proxy-groups";
import { getClashConfig, updateConfigs } from "@/services/api";
import { Button, ButtonGroup } from "@nextui-org/react";
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
          <ButtonGroup color="primary" size="sm" variant="bordered">
            {modeList.map((mode) => (
              <Button
                key={mode}
                variant={mode === "rule" ? "solid" : "bordered"}
                onClick={() => onChangeMode(mode)}
                className="capitalize"
              >
                {t(mode)}
              </Button>
            ))}
          </ButtonGroup>
        </div>
      }
    >
      <ProxyGroups mode={curMode!} />
    </BasePage>
  );
}
