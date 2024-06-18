"use client";

import { useTheme } from "next-themes";
import { DarkIcon, LightIcon, Logo } from ".";

const Sidebar = () => {
  const { theme } = useTheme();

  return (
    <div className="basis-[200px] grow shrink-0 w-full h-full flex flex-col">
      <div className="flex flex-row">
        <div className="flex h-[27px] justify-between">
          {theme === "dark" ? (
            <DarkIcon fill="black" className="w-8 h-8" />
          ) : (
            <LightIcon fill="black" className="w-8 h-8" />
          )}
        </div>
        <Logo fill="black" />
      </div>
    </div>
  );
};

export default Sidebar;
