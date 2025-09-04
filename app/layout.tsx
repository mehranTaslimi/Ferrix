'use client';

import { type } from '@tauri-apps/plugin-os';
import { clsx } from 'clsx';
import { useLayoutEffect, useState } from 'react';

import { DownloadProvider } from '@/components/download-context';
import { PresetsThemeProvider } from '@/components/preset-theme/preset-theme-context';
import AppSidebar from '@/components/sidebar';
import { ThemeProvider } from '@/components/theme-provider';
import { SidebarProvider } from '@/components/ui/sidebar';
import { Toaster } from '@/components/ui/sonner';

import type { OsType } from '@tauri-apps/plugin-os';
import type React from 'react';

import './globals.css';

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const [OS, setOS] = useState<OsType>();

  useLayoutEffect(() => {
    setOS(type());
  }, []);

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
                     if (preset && preset !== 'default') {
                     document.documentElement.style.visibility = 'hidden';
                     }
                  } catch (e) {}
                     })();
                        `,
          }}
        />
      </head>
      <body
        className={clsx('antialiased', {
          'bg-sidebar': OS === 'linux' || OS === 'windows',
          'bg-background/30': OS === 'macos',
        })}
      >
        <PresetsThemeProvider>
          <ThemeProvider
            attribute="class"
            defaultTheme="system"
            enableSystem
            disableTransitionOnChange
            storageKey="theme"
            themes={['light', 'dark']}
          >
            <DownloadProvider>
              <div className="h-screen overflow-hidden">
                <div className="h-full">
                  <div
                    className="fixed top-0 left-0 z-50 h-6 w-full bg-transparent"
                    data-tauri-drag-region
                  />
                  <SidebarProvider className="h-screen w-screen">
                    <AppSidebar />

                    <div className="mr-2 flex h-screen w-full flex-col items-center justify-center">
                      <div className="bg-background h-[97.4%] w-full overflow-x-hidden overflow-y-auto rounded-lg pb-3">
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
  );
}
