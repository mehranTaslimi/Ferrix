"use client"

import {
   createContext,
   useContext,
   useState,
   useMemo,
   useLayoutEffect,
   useEffect,
} from "react"
import { defaultPresets } from "@/utils/theme/theme-presets"
import { themePresetToCss } from "@/utils/theme/theme-helpers"

interface CustomThemeContextType {
   preset: string
   setPreset: (preset: string) => void
   tempPreset: string
   removeTempPreset: () => void
   addTempPreset: (preset: string) => void
   applyTempPreset: (preset: string) => void
}

const CustomThemeContext = createContext<CustomThemeContextType | undefined>(
   undefined
)

export function useThemePreset() {
   const context = useContext(CustomThemeContext)
   if (context === undefined) {
      throw new Error("useThemePreset must be used within a ThemeProvider")
   }
   return context
}

export function PresetsThemeProvider({
   children,
}: {
   children: React.ReactNode
}) {
   const [preset, setPreset] = useState(() => {
      if (typeof window === "undefined") {
         return "modern-minimal"
      }
      return localStorage.getItem("theme-preset") || "modern-minimal"
   })
   const [tempPreset, setTempPreset] = useState("")

   const handlePresetChange = (preset: string) => {
      const theme = defaultPresets[preset]
      if (theme) {
         const css = themePresetToCss(theme)
         const styleTagId = "dynamic-theme-preset-style"
         let styleTag = document.getElementById(
            styleTagId
         ) as HTMLStyleElement | null

         if (!styleTag) {
            styleTag = document.createElement("style")
            styleTag.id = styleTagId
            document.head.appendChild(styleTag)
         }

         styleTag.innerHTML = css
         localStorage.setItem("theme-preset", preset)

         document.documentElement.style.visibility = "visible"
      }
   }

   const applyTempPreset = (preset: string) => {
      setPreset(preset)
   }

   const addTempPreset = (preset: string) => {
      const theme = defaultPresets[preset]
      if (theme) {
         const css = themePresetToCss(theme)
         const styleTagId = "temp-theme-preset-style"
         let styleTag = document.getElementById(
            styleTagId
         ) as HTMLStyleElement | null

         if (!styleTag) {
            styleTag = document.createElement("style")
            styleTag.id = styleTagId
            document.head.appendChild(styleTag)
         }

         styleTag.innerHTML = css
         setTempPreset(preset)
      }
   }

   const removeTempPreset = () => {
      const styleTagId = "temp-theme-preset-style"
      const styleTag = document.getElementById(
         styleTagId
      ) as HTMLStyleElement | null

      if (styleTag) {
         styleTag.remove()
      }
      setTempPreset("")
   }

   useLayoutEffect(() => {
      handlePresetChange(preset)
   }, [preset])

   const value = useMemo(
      () => ({
         preset,
         setPreset,
         tempPreset,
         removeTempPreset,
         addTempPreset,
         applyTempPreset,
      }),
      [preset, tempPreset]
   )

   return (
      <CustomThemeContext.Provider value={value}>
         {children}
      </CustomThemeContext.Provider>
   )
}
