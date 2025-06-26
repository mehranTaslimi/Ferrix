"use client";

import { ThemeProvider } from "@/components/theme-provider";
import "./globals.css";


export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
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
          <div className="h-screen overflow-hidden">
            <div className="h-full overflow-y-auto">
              <div
                className="fixed w-full h-8 bg-transparent left-0 top-0 z-auto"
                data-tauri-drag-region
              />
              <div className="flex w-screen h-screen">
                <div className="basis-2xs overflow-x-hidden overflow-y-auto p-8">
                  Hello
                </div>
                <div className="bg-background flex-1 overflow-x-hidden overflow-y-auto p-8">
                  {children}
                </div>
              </div>
            </div>
          </div>
        </ThemeProvider>
      </body>
    </html>
  );
}
