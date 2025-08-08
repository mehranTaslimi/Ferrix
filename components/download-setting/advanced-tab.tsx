"use client";

import { useEffect, useMemo, useState } from "react";
import { TabsContent } from "@radix-ui/react-tabs";
import { FormControl, FormField, FormItem, FormLabel } from "../ui/form";
import { Input } from "../ui/input";
import type { UseFormReturn } from "react-hook-form";
import type { DownloadFormData } from "./download-setting-sheet";
import PositiveNumberField from "./positive-number-field";
import KeyValuePairField from "./key-value-pair-field";
import FormMessage from "./form-message";
import { Button } from "../ui/button";
import { FolderOpen } from "lucide-react";
import { open } from "@tauri-apps/plugin-dialog";
import { platform as getPlatform } from "@tauri-apps/plugin-os";

interface AdvancedTabProps {
  form: UseFormReturn<DownloadFormData>;
  handleKeyPress: (e: React.KeyboardEvent) => void;
}

export default function AdvancedTab({ form, handleKeyPress }: AdvancedTabProps) {
  const [os, setOs] = useState<"windows" | "macos" | "linux" | "unknown">("unknown");

  useEffect(() => {
    (async () => {
      try {
        const p = await getPlatform();
        if (p === "windows" || p === "macos" || p === "linux") setOs(p);
      } catch {
        setOs("unknown");
      }
    })();
  }, []);

  const placeholder = useMemo(() => {
    if (os === "windows") return "e.g. C:\\Users\\YourName\\Downloads";
    if (os === "macos") return "e.g. /Users/yourname/Downloads";
    if (os === "linux") return "e.g. /home/yourname/Downloads";
    return "Select a folder…";
  }, [os]);

  const handleSelectDirectory = async () => {
    try {
      const selected = await open({ directory: true, multiple: false, title: "Select Download Directory" });
      if (typeof selected === "string") {
        form.setValue("filePath", selected, { shouldValidate: true, shouldDirty: true });
      } else if (Array.isArray(selected) && selected[0]) {
        form.setValue("filePath", selected[0], { shouldValidate: true, shouldDirty: true });
      }
    } catch (error) {
      console.error("Error selecting directory:", error);
      form.setError("filePath", {
        type: "manual",
        message: "Failed to select directory. Please try again.",
      });
    }
  };

  return (
    <TabsContent value="advanced" className="space-y-3 p-2">
      {/* Download location */}
      <FormField
        control={form.control}
        name="filePath"
        render={({ field }) => (
          <FormItem className="gap-1 flex-col">
            <FormLabel htmlFor="filePath">Download location</FormLabel>
            <div className="flex gap-2">
              <FormControl>
                <Input id="filePath" disabled {...field} placeholder={placeholder} className="flex-1" />
              </FormControl>
              <Button type="button" variant="outline" onClick={handleSelectDirectory} className="shrink-0">
                <FolderOpen className="w-4 h-4 mr-2" />
                Browse
              </Button>
            </div>
            <FormMessage />
          </FormItem>
        )}
      />

      {/* Optional: Speed limit (uncomment when backend supports) */}
      {/* <SpeedLimitField form={form} handleKeyPress={handleKeyPress} /> */}

      {/* Retries / backoff / timeout */}
      <PositiveNumberField form={form} name="maxRetries" label="Max retries" min={0} max={30} defaultValue={3} />
      <PositiveNumberField form={form} name="backoffFactor" label="Backoff factor" min={1} max={10} defaultValue={2} />
      <PositiveNumberField form={form} name="timeoutSecs" label="Timeout (seconds)" min={10} max={120} defaultValue={30} />

      {/* Advanced headers/cookies */}
      <KeyValuePairField form={form} name="headers" label="Headers" handleKeyPress={handleKeyPress} />
      <KeyValuePairField form={form} name="cookies" label="Cookies" handleKeyPress={handleKeyPress} />
    </TabsContent>
  );
}
