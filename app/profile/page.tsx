import BasePage from "@/components/layout/base/base-page";
import { ProfileItem } from "@/components/profile/profile-item";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useProfiles } from "@/hooks/use-profiles";
import { importProfile } from "@/services/cmds";
import { DndContext } from "@dnd-kit/core";
import { SortableContext } from "@dnd-kit/sortable";

import { FileText, RefreshCcw } from "lucide-react";
import { useTranslations } from "next-intl";
import { useMemo, useState } from "react";

export default function Page() {
  const t = useTranslations();

  const [url, setUrl] = useState("");
  const [loading, setLoading] = useState(false);
  const [disabled, setDisabled] = useState(false);

  const { profiles = {} } = useProfiles();

  const { regularItems, enhanceItems } = useMemo(() => {
    const items = profiles.items || [];
    const chain = profiles.chain || [];

    const type1 = ["local", "remote"];
    const type2 = ["merge", "script"];

    const regularItems = items.filter((i) => i && type1.includes(i.type!));
    const restItems = items.filter((i) => i && type2.includes(i.type!));
    const restMap = Object.fromEntries(restItems.map((i) => [i.uid, i]));

    const enhanceItems = chain
      .map((i) => restMap[i]!)
      .filter(Boolean)
      .concat(restItems.filter((i) => !chain.includes(i.uid)));

    return { regularItems, enhanceItems };
  }, [profiles]);

  const onImport = async () => {
    if (!url) return;

    setLoading(true);

    try {
      await importProfile(url);
    } catch (error) {
      setLoading(false);
    } finally {
      setLoading(false);
      setDisabled(false);
    }
  };

  return (
    <BasePage
      title={t("Profiles")}
      header={
        <div className="flex items-center gap-1">
          <Button variant="ghost" size="sm">
            <RefreshCcw />
          </Button>

          <Button variant="ghost" size="sm">
            <FileText />
          </Button>
        </div>
      }
    >
      <div className="flex gap-1 mx-3 h-9 items-center py-8">
        <Input
          type="text"
          value={url}
          placeholder={t("Profile URL")}
          onChange={(e) => setUrl(e.target.value)}
        />
        <Button onClick={onImport} disabled={disabled}>
          {t("Import")}
        </Button>
        <Button>{t("New")}</Button>
      </div>
      <div className="pt-1 mb-1 pl-2 mr-2">
        <DndContext>
          <SortableContext
            items={regularItems.map((x) => {
              return x.uid;
            })}
          >
            {regularItems.map((item) => (
              <ProfileItem
                key={item.file}
                id={item.uid}
                selected={profiles.current === item.uid}
                itemData={item}
              />
            ))}
          </SortableContext>
        </DndContext>
      </div>
    </BasePage>
  );
}
