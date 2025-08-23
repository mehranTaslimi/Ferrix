"use client";

import type React from "react";
import { ThemeProvider } from "@/components/theme-provider";
import { DownloadProvider } from "@/components/download-context";
import AppSidebar from "@/components/sidebar";
import "./globals.css";
import { Toaster } from "@/components/ui/sonner";
import { SidebarProvider } from "@/components/ui/sidebar";
import { OsType, type } from "@tauri-apps/plugin-os";
import clsx from "clsx";
import { useLayoutEffect, useState } from "react";
import {WindowsTitlebar, TITLE_BAR_HEIGHT} from "@/components/windows-titlebar";

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const [OS, setOS] = useState<OsType>()


  useLayoutEffect(() => {
    setOS(type())
  }, []);
  
  const windowContainerHeight = OS === 'windows' ? `calc(100vh - ${TITLE_BAR_HEIGHT}px - 8px)` : '100vh';

  return (
    <html lang="en" suppressHydrationWarning>
      <body className={clsx("antialiased h-screen w-screen", {
        "bg-secondary": OS === 'linux' || OS === 'windows',
        "bg-background/30": OS === 'macos',
      })}>
        <ThemeProvider
          attribute="class"
          defaultTheme="system"
          enableSystem
          disableTransitionOnChange
          storageKey="theme"
          themes={["light", "dark"]}
        >
          <WindowsTitlebar/>
          <DownloadProvider>
            <div className="overflow-hidden">
                <SidebarProvider className="w-screen">
                  <AppSidebar />
                  <div className="flex items-center justify-start flex-col mr-2 w-full" style={{
                    height: windowContainerHeight
                  }}>
                    <div className="bg-background w-full h-full rounded-lg overflow-x-hidden pb-3 overflow-y-auto">
                      {children}
                      <Toaster />
                    </div>
                  </div>
                </SidebarProvider>
            </div>
          </DownloadProvider>
        </ThemeProvider>
      </body>
    </html>
  );
}
