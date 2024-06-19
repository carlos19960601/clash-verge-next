"use client";

import { ClashLayout } from "@/components";
import { NextUIProvider } from "@nextui-org/react";
import { ThemeProvider } from "next-themes";
import { PropsWithChildren } from "react";
import { SWRConfig } from "swr";

const Providers = ({ children }: PropsWithChildren) => {
  return (
    <NextUIProvider>
      <ThemeProvider attribute="class" defaultTheme="dark">
        <SWRConfig value={{ errorRetryCount: 3 }}>
          <ClashLayout>{children}</ClashLayout>
        </SWRConfig>
      </ThemeProvider>
    </NextUIProvider>
  );
};

export default Providers;
