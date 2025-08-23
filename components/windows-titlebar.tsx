"use client"

import { useEffect, useState } from "react"
import Image from "next/image"
import { getCurrentWindow } from "@tauri-apps/api/window"
import { Square, X, Minus } from "lucide-react"
import { useTheme } from "next-themes"
import { isWindows } from "@/utils/os-utils"

export const TITLE_BAR_HEIGHT = 32;

export function WindowsTitlebar() {
   const [isMounted, setIsMounted] = useState(false)
   useEffect(() => {
      setIsMounted(true)
   }, [])

   const { theme, systemTheme } = useTheme()
   const shouldImageInvert =
      theme === "light" || (theme === "system" && systemTheme === "light")

   if (!isMounted || !isWindows) {
      return null
   }

   const appWindow = getCurrentWindow()
   const handleMinimize = () => {
      appWindow
         .minimize()
         .catch((e) => console.error("Failed to minimize window:", e))
   }
   const handleMaximize = () => {
      appWindow
         .toggleMaximize()
         .catch((e) => console.error("Failed to maximize window:", e))
   }
   const handleClose = () => {
      appWindow
         .close()
         .catch((e) => console.error("Failed to close window:", e))
   }

   return (
      <div
         style={{ height: `${TITLE_BAR_HEIGHT}px` }}
         className="titlebar flex w-full items-center justify-between z-[50] relative"
         data-tauri-drag-region
      >
         <div className="flex items-center px-2 py-1 gap-2">
            <Image
               width={18}
               height={18}
               src="/logo.png"
               alt="Ferrix"
               className={shouldImageInvert ? "filter invert" : ""}
            />
            <p className="text-sm">Ferrix</p>
         </div>
         <div className="flex items-center">
            <button
               id="titlebar-minimize"
               className="p-2 transition-colors hover:bg-muted/50 z-[60]"
               onClick={handleMinimize}
            >
               <Minus width={12} height={16} />
            </button>
            <button
               id="titlebar-maximize"
               className="p-2 transition-colors hover:bg-muted/50 z-[60]"
               onClick={handleMaximize}
            >
               <Square width={12} height={16} opacity='0.5' />
            </button>
            <button
               id="titlebar-close"
               className="p-2 group transition-colors hover:bg-destructive hover:text-foreground z-[60]"
               onClick={handleClose}
            >
               <X
                  width={16}
                  height={16}
                  opacity='0.5'
                  className="group-hover:text-foreground"
               />
            </button>
         </div>
      </div>
   )
}
