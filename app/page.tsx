import BasePage from "@/components/layout/base/base-page";
import { Button, ButtonGroup } from "@nextui-org/react";
import { useTranslations } from "next-intl";

export default function Home() {
  const modeList = ["rule", "global", "direct"];
  const t = useTranslations();

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
              >
                {t(mode)}
              </Button>
            ))}
          </ButtonGroup>
        </div>
      }
    ></BasePage>
  );
}
