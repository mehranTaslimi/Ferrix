import { TabsContent } from "@radix-ui/react-tabs";
import { FormControl, FormField, FormItem, FormLabel } from "../ui/form";
import { Input } from "../ui/input";
import { UseFormReturn } from "react-hook-form";
import { DownloadFormData } from "./download-setting-sheet";
import PositiveNumberField from "./positive-number-field";
import { open } from "@tauri-apps/plugin-dialog";
import { Button } from "../ui/button";
import { FolderOpen } from "lucide-react";
import KeyValuePairField from "./key-value-pair-field";
import FormMessage from "./form-message";

interface AdvancedTabProps {
  form: UseFormReturn<DownloadFormData>;
  handleKeyPress: (e: React.KeyboardEvent) => void;
}

export default function AdvancedTab({
  form,
  handleKeyPress,
}: AdvancedTabProps) {
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
    <TabsContent value="advanced" className="space-y-4 overflow-scroll p-2">
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

      <KeyValuePairField
        name="headers"
        label="Headers"
        form={form}
        handleKeyPress={handleKeyPress}
      />
      <KeyValuePairField
        name="cookies"
        label="Cookies"
        form={form}
        handleKeyPress={handleKeyPress}
      />

      {/* <SpeedLimitField form={form} /> */}
      <PositiveNumberField
        max={30}
        defaultValue={3}
        min={0}
        form={form}
        name="maxRetries"
        label="Max Retries"
      />
      <PositiveNumberField
        min={2}
        defaultValue={2}
        max={10}
        form={form}
        name="backoffFactor"
        label="Backoff Factor"
      />
      <PositiveNumberField
        min={10}
        max={120}
        defaultValue={30}
        form={form}
        name="timeoutSecs"
        label="Timeout Duration (secs)"
      />
    </TabsContent>
  );
}
