"use client"

import type React from "react"
import { ThemeProvider } from "@/components/theme-provider"
import { DownloadProvider } from "@/components/download-context"
import AppSidebar from "@/components/sidebar"
import "./globals.css"
import { Toaster } from "@/components/ui/sonner"
import { SidebarProvider } from "@/components/ui/sidebar"
import { OsType, type } from "@tauri-apps/plugin-os"
import clsx from "clsx"
import { useLayoutEffect, useState } from "react"
import { PresetsThemeProvider } from "@/components/presets-theme-context"

export default function RootLayout({
   children,
}: Readonly<{
   children: React.ReactNode
}>) {
   const [OS, setOS] = useState<OsType>()

   useLayoutEffect(() => {
      setOS(type())
   }, [])

   return (
      <html lang="en" suppressHydrationWarning>
         <head>
            <script
               dangerouslySetInnerHTML={{
                  __html: `
                  (function() {
                     try {
                     var preset = localStorage.getItem('theme-preset');
                     // If a preset is stored and it's not the default, hide the page
                     if (preset && preset !== 'modern-minimal') {
                     document.documentElement.style.visibility = 'hidden';
                     }
                  } catch (e) {}
                     })();
                        `,
               }}
            />
         </head>
         <body className="antialiased">
            <PresetsThemeProvider>
               <ThemeProvider
                  attribute="class"
                  defaultTheme="system"
                  enableSystem
                  disableTransitionOnChange
                  storageKey="theme"
                  themes={["light", "dark"]}
               >
                  <DownloadProvider>
                     <div
                        className={clsx("h-screen overflow-hidden", {
                           "bg-secondary/30":
                              OS === "linux" || OS === "windows",
                           "bg-background/30": OS === "macos",
                        })}
                     >
                        <div className="h-full">
                           <SidebarProvider className="flex gap-2 w-screen h-screen">
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
            </PresetsThemeProvider>
         </body>
      </html>
   )
}
