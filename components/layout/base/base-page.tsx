"use client";

import useIsDark from "@/hooks/useIsDark";
import { cn } from "@nextui-org/react";
import { PropsWithChildren, ReactNode } from "react";
import { BaseErrorBoundary } from "./base-error-boundary";

interface Props {
  title?: ReactNode;
  header?: ReactNode;
  full?: boolean;
  contentStyle?: string;
}
const BasePage = (props: PropsWithChildren<Props>) => {
  const isDark = useIsDark();
  return (
    <BaseErrorBoundary>
      <div className="w-full h-full flex flex-col">
        <header
          className={cn(
            "select-none flex py-5 items-center justify-between border-1 border-solid",
            isDark ? "border-white/5" : "border-black/5"
          )}
        >
          <h1 className="text-2xl font-bold">{props.title}</h1>
          {props.header}
        </header>

        <div
          className={cn(
            "h-full overflow-hidden",
            isDark ? "bg-zinc-900" : "bg-white"
          )}
        >
          <section className={cn(props.full && "p-0 overflow-visible")}>
            <div
              className={cn(
                props.full ? "w-full" : "m-auto w-[calc(100% - 10px * 2)]",
                props.contentStyle
              )}
            >
              {props.children}
            </div>
          </section>
        </div>
      </div>
    </BaseErrorBoundary>
  );
};

export default BasePage;
