import type { Metadata } from "next";
import { getLocale, getMessages } from "next-intl/server";
import { Inter as FontSans } from "next/font/google";
import "./globals.css";
import Providers from "./providers";

import { cn } from "@/lib/utils";
import { NextIntlClientProvider } from "next-intl";

export const metadata: Metadata = {
  title: "Clash Verge Next",
  description: "Clash GUI",
};

const fontSans = FontSans({
  subsets: ["latin"],
  variable: "--font-sans",
});

export default async function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const locale = await getLocale();

  // Providing all messages to the client
  // side is the easiest way to get started
  const messages = await getMessages();

  return (
    <html lang={locale} suppressHydrationWarning>
      <body
        className={cn(
          "min-h-screen bg-background text-foreground font-sans antialiased",
          fontSans.variable
        )}
      >
        <NextIntlClientProvider messages={messages}>
          <Providers>{children}</Providers>
        </NextIntlClientProvider>
      </body>
    </html>
  );
}
