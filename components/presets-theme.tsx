'use client'

import {
   Dialog,
   DialogClose,
   DialogContent,   
   DialogFooter,
   DialogHeader,
   DialogTitle,
   DialogTrigger,
} from "@/components/ui/dialog"
import { Button } from "./ui/button"
import { Label } from "./ui/label"
import { Input } from "./ui/input"
import { defaultPresets } from "@/utils/theme/theme-presets"
import { getColorPalette } from "@/utils/theme/get-colors-palette"
import { useThemePreset } from "./presets-theme-context"
import { Search, Check } from "lucide-react"
import clsx from "clsx"
import { useState } from "react"

export function PresetThemes() {
   const {
      preset,
      tempPreset,
      addTempPreset,
      removeTempPreset,
      applyTempPreset,
   } = useThemePreset()
   const [search, setSearch] = useState("")

   const filteredPresets = Object.entries(defaultPresets).filter(([key, value]) =>
      value.label?.toLowerCase().includes(search.toLowerCase())
   )

   return (
      <Dialog onOpenChange={(isOpen) => !isOpen && removeTempPreset()}>
         <form>
            <DialogTrigger asChild>
               <Button variant="outline">Change preset</Button>
            </DialogTrigger>
            <DialogContent className="flex flex-col gap-4 justify-between sm:max-w-[425px] h-[500px]">
               <div>
                  <DialogHeader>
                     <DialogTitle>Change your preset</DialogTitle>
                  </DialogHeader>
                  <div className="grid gap-4 mt-4">
                     <div className="flex items-center gap-2 border rounded-lg px-2 focus-within:ring-2 focus-within:ring-ring transition">
                     <Search className="w-4 h-4 text-muted-foreground" />
                     <Input
                        className="focus-visible:ring-0 border-0 px-0 bg-transparent!"
                        id="search_presets"
                        name="search_presets"
                        value={search}
                        onChange={(e) => setSearch(e.target.value)}
                        placeholder="Search presets..."
                     />
                     </div>
                     <div className="overflow-y-auto h-[320px] space-y-2">
                        {filteredPresets.map(([key, value]) => (
                           <div
                              key={key}
                              className={clsx(
                                 "flex items-center justify-between p-2 rounded-lg hover:bg-muted",
                                 {
                                    "bg-muted": tempPreset === key,
                                 }
                              )}
                              onClick={() => addTempPreset(key)}
                           >
                              <div className="flex items-center gap-2">
                                 {getColorPalette(key)}
                                 <span aria-label={value.label}>
                                    {value.label}
                                 </span>
                              </div>
                              {preset === key && (
                                 <Check className="h-4 w-4 text-muted-foreground" />
                              )}
                           </div>
                        ))}
                     </div>
                  </div>
               </div>
               <DialogFooter>
                  <DialogClose asChild>
                     <Button variant="outline" onClick={removeTempPreset}>
                        Cancel
                     </Button>
                  </DialogClose>
                  <DialogClose asChild>
                     <Button
                        type="submit"
                        onClick={() => applyTempPreset(tempPreset)}
                     >
                        Save changes
                     </Button>
                  </DialogClose>
               </DialogFooter>
            </DialogContent>
         </form>
      </Dialog>
   )
}
