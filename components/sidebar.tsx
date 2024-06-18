"use client";

import { useTheme } from "next-themes";
import { DarkIcon, LightIcon, Logo } from ".";

const Sidebar = () => {
  const { theme } = useTheme();

  return (
    <div className="basis-[200px] grow shrink-0 w-full h-full flex flex-col">
      <div className="flex flex-row">
        <div className="w-8 h-8">
          {theme === "dark" ? <LightIcon /> : <DarkIcon />}
        </div>
        <Logo />
      </div>
    </div>
  );
};

export default Sidebar;
