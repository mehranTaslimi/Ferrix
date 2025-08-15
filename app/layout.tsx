import type React from "react";
import { ThemeProvider } from "@/components/theme-provider";
import { DownloadProvider } from "@/components/download-context";
import AppSidebar from "@/components/sidebar";
import "./globals.css";
import { Toaster } from "@/components/ui/sonner";
import { SidebarProvider } from "@/components/ui/sidebar";

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body className="antialiased bg-background/30">
        <ThemeProvider
          attribute="class"
          defaultTheme="system"
          enableSystem
          disableTransitionOnChange
          storageKey="theme"
          themes={["light", "dark"]}
        >
          <DownloadProvider>
            <div className="h-screen overflow-hidden">
              <div className="h-full">
                <div
                  className="fixed w-full h-6 bg-transparent left-0 top-0 z-50"
                  data-tauri-drag-region
                />
                <SidebarProvider className="w-screen h-screen">
                  <AppSidebar />

                  <div className="h-screen flex items-center justify-center flex-col mr-2 w-full">
                    <div className="bg-background w-full h-[97.4%] rounded-lg overflow-x-hidden pb-3 overflow-y-auto">
                      {children}
                      <Toaster />
                    </div>
                  </div>
                </SidebarProvider>
              </div>
            </div>
          </DownloadProvider>
        </ThemeProvider>
      </body>
    </html>
  );
}
