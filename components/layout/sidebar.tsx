"use client";

import { cn } from "@/lib/utils";
import {
  Cable,
  Podcast,
  Scale,
  ScrollText,
  Settings,
  Shell,
  Zap,
} from "lucide-react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import IconIcon from "../svgs/icon.svg";
import IconLogo from "../svgs/logo.svg";
import Traffic from "./traffic";

const menus = [
  {
    href: "/",
    label: "代理",
    icon: <Shell size={18} />,
  },
  {
    href: "/profile",
    label: "订阅",
    icon: <Podcast size={18} />,
  },
  {
    href: "/connections",
    label: "连接",
    icon: <Cable size={18} />,
  },
  {
    href: "/rules",
    label: "规则",
    icon: <Scale size={18} />,
  },
  {
    href: "/logs",
    label: "日志",
    icon: <ScrollText size={18} />,
  },
  {
    href: "/test",
    label: "测试",
    icon: <Zap size={18} />,
  },
  {
    href: "/settings",
    label: "设置",
    icon: <Settings size={18} />,
  },
];

const Sidebar = () => {
  const pathname = usePathname();
  const isActive = (href: string) => {
    if (href === "/") {
      return pathname === "/";
    }
    return pathname.includes(href);
  };

  return (
    <div className="bg-muted/40">
      <div className="">
        <div className="flex h-16 items-center justify-center space-x-2 px-2">
          <IconIcon className="fill-primary w-6" />
          <IconLogo className="fill-primary w-24" />
        </div>

        <nav className="flex flex-col px-2 items-start text-sm font-medium">
          {menus.map((menu) => (
            <Link
              key={menu.href}
              href={menu.href}
              className={cn(
                "w-full flex items-center gap-8 px-4 rounded-lg py-2 transition-all text-muted-foreground hover:text-primary",
                isActive(menu.href) && "text-primary bg-muted"
              )}
            >
              {menu.icon}
              <span>{menu.label}</span>
            </Link>
          ))}
        </nav>

        <div className="">
          <Traffic />
        </div>
      </div>
    </div>
  );
};

export default Sidebar;
