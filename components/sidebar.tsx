"use client";

import { useTheme } from "next-themes";
import { DarkIcon, LightIcon, Logo } from ".";

const Sidebar = () => {
  const { theme } = useTheme();

  return (
    <div className="basis-[200px] grow shrink-0 w-full h-full flex flex-col">
      <div className="flex flex-row">
        <div>
          {theme === "dark" ? (
            <LightIcon className="w-8 h-8" />
          ) : (
            <DarkIcon className="w-8 h-8" />
          )}
        </div>
        <Logo />
      </div>
    </div>
  );
};

export default Sidebar;
