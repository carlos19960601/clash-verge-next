"use client";

import { PropsWithChildren } from "react";
import { SWRConfig } from "swr";

const Providers = ({ children }: PropsWithChildren) => {
  return (
    <SWRConfig value={{ errorRetryCount: 3 }}>
      <div className="h-full w-full">
        <div></div>
        <div>{children}</div>
      </div>
    </SWRConfig>
  );
};

export default Providers;
