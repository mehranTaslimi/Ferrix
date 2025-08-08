import { TabsContent } from "@radix-ui/react-tabs";
import { FormControl, FormField, FormItem, FormLabel } from "../ui/form";
import { Input } from "../ui/input";
import { useFormContext, UseFormReturn } from "react-hook-form";
import { DownloadFormData } from "./download-setting-sheet";
import PositiveNumberField from "./positive-number-field";
import { open } from "@tauri-apps/plugin-dialog";
import { Button } from "../ui/button";
import { FolderOpen } from "lucide-react";
import KeyValuePairField from "./key-value-pair-field";
import FormMessage from "./form-message";
import ProxyField from "./proxy-field";
import AuthField from "./auth-field";

interface AdvancedTabProps {
  handleKeyPress: (e: React.KeyboardEvent) => void;
}

export default function AdvancedTab({ handleKeyPress }: AdvancedTabProps) {
  const form = useFormContext();
  const handleSelectDirectory = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select Download Directory",
      });

      if (selected) {
        form.setValue("filePath", selected);
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
    <TabsContent value="advanced" className="space-y-3 overflow-scroll p-2">
      <FormField
        control={form.control}
        name="filePath"
        render={({ field }) => (
          <FormItem className="gap-1 flex-col">
            <FormLabel htmlFor="filePath">Download Location</FormLabel>
            <div className="flex gap-2">
              <FormControl>
                <Input
                  disabled
                  {...field}
                  placeholder={
                    process.platform === "win32"
                      ? "e.g. C:\\Users\\YourName\\Downloads"
                      : "e.g. /Users/yourname/Downloads"
                  }
                  className="flex-1"
                />
              </FormControl>
              <Button
                type="button"
                variant="outline"
                onClick={handleSelectDirectory}
                className="shrink-0"
              >
                <FolderOpen className="w-4 h-4 mr-2" />
                Browse
              </Button>
            </div>
            <FormMessage />
          </FormItem>
        )}
      />
      <ProxyField />
      <AuthField />
      <KeyValuePairField
        name="headers"
        label="Headers"
        handleKeyPress={handleKeyPress}
      />
      <KeyValuePairField
        name="cookies"
        label="Cookies"
        handleKeyPress={handleKeyPress}
      />
      {/* <SpeedLimitField form={form} /> */}
      <PositiveNumberField
        max={30}
        defaultValue={3}
        min={0}
        name="maxRetries"
        label="Max Retries"
      />
      <PositiveNumberField
        min={2}
        defaultValue={2}
        max={10}
        name="backoffFactor"
        label="Backoff Factor"
      />
      <PositiveNumberField
        min={10}
        max={120}
        defaultValue={30}
        name="timeoutSecs"
        label="Timeout Duration (secs)"
      />
    </TabsContent>
  );
}
