import type React from "react"
import { ThemeProvider } from "@/components/theme-provider"
import { DownloadProvider } from "@/components/download-context"
import Sidebar from "@/components/sidebar"
import "./globals.css"
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "@/components/ui/resizable"

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body className="antialiased">
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
              <div className="h-full overflow-y-auto">
                <div className="fixed w-full h-6 bg-transparent left-0 top-0 z-50" data-tauri-drag-region />
                <div className="w-screen h-screen">
                  <ResizablePanelGroup direction="horizontal">
                    <ResizablePanel defaultSize={25} minSize={10} maxSize={40}>
                      <div className="overflow-x-hidden overflow-y-auto">
                        <Sidebar />
                      </div>
                    </ResizablePanel>
                    <ResizableHandle className="bg-transparent" />
                    <ResizablePanel defaultSize={75}>
                      <div className="h-screen flex items-center justify-center flex-col mr-3">
                        <div className="bg-background w-full h-[96%] rounded-md p-3 overflow-x-hidden overflow-y-auto">{children}</div>
                      </div>
                    </ResizablePanel>
                  </ResizablePanelGroup>
                </div>
              </div>
            </div>
          </DownloadProvider>
        </ThemeProvider>
      </body>
    </html>
  )
}
