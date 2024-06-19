"use client";

import useIsDark from "@/hooks/useIsDark";
import { Listbox, ListboxItem, Selection, cn } from "@nextui-org/react";
import {
  Cable,
  Podcast,
  Scale,
  ScrollText,
  Settings,
  Shell,
  Zap,
} from "lucide-react";
import { useTheme } from "next-themes";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { useMemo, useState } from "react";
import { IconIcon, IconLogo } from "..";
import Traffic from "./traffic";

const menus = [
  {
    href: "/",
    label: "代理",
    icon: <Shell size={18} />,
  },
  {
    href: "/subscription",
    label: "订阅",
    icon: <Podcast size={18} />,
  },
  {
    href: "/connection",
    label: "连接",
    icon: <Cable size={18} />,
  },
  {
    href: "/rule",
    label: "规则",
    icon: <Scale size={18} />,
  },
  {
    href: "/log",
    label: "日志",
    icon: <ScrollText size={18} />,
  },
  {
    href: "/test",
    label: "测试",
    icon: <Zap size={18} />,
  },
  {
    href: "/setting",
    label: "设置",
    icon: <Settings size={18} />,
  },
];

const Sidebar = () => {
  const { theme } = useTheme();

  const fillColor = useMemo(() => {
    return theme === "dark" ? "white" : "black";
  }, [theme]);

  const isDark = useIsDark();

  const [selectedKeys, setSelectedKeys] = useState<Selection>(
    new Set(["proxy"])
  );

  const pathname = usePathname();

  const isActive = (href: string) => {
    if (href === "/") {
      return pathname === "/";
    }
    return pathname.includes(href);
  };

  return (
    <div
      className={cn(
        "basis-[200px] grow shrink-0 w-full h-full flex flex-col pb-2 select-none border-r-1 border-solid",
        isDark ? "border-white/5" : "border-black/5"
      )}
    >
      <div className="relative flex flex-col grow shrink-0 basis-[58px] w-full py-5 justify-center">
        <div className="flex justify-between h-[27px] ">
          <IconIcon
            // @ts-ignore
            fill={fillColor}
            className="w-9 h-9 mt-[-3px] mr-[5px] ml-[-5px]"
          />
          <IconLogo
            // @ts-ignore
            fill={fillColor}
          />
        </div>
        <div className="grow shrink basis-4/5 overflow-y-auto pt-1">
          <Listbox
            selectionMode="single"
            selectedKeys={selectedKeys}
            onSelectionChange={setSelectedKeys}
            variant="flat"
            hideSelectedIcon={true}
          >
            {menus.map((menu) => (
              <ListboxItem
                as={Link}
                key={menu.href}
                href={menu.href}
                startContent={menu.icon}
                classNames={{
                  base: cn(isActive(menu.href) && "bg-primary text-white"),
                }}
              >
                {menu.label}
              </ListboxItem>
            ))}
          </Listbox>
        </div>

        <div className="">
          <Traffic />
        </div>
      </div>
    </div>
  );
};

export default Sidebar;
