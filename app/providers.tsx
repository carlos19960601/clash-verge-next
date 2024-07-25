"use client";

import ClashLayout from "@/components/layout/clash-layout";
import { ThemeProvider } from "next-themes";
import { PropsWithChildren } from "react";
import { SWRConfig } from "swr";

const Providers = ({ children }: PropsWithChildren) => {
  return (
    <ThemeProvider attribute="class" defaultTheme="dark">
      <SWRConfig value={{ errorRetryCount: 3 }}>
        <ClashLayout>{children}</ClashLayout>
      </SWRConfig>
    </ThemeProvider>
  );
};

export default Providers;
