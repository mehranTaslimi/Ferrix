import { TabsContent } from "@radix-ui/react-tabs";
import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "../ui/form";
import { Input } from "../ui/input";
import { UseFormReturn } from "react-hook-form";
import { DownloadFormData } from "./download-setting-sheet";
import PositiveNumberField from "./positive-number-field";
import SpeedLimitField from "./Speed-limit-field";

interface AdvancedTabProps {
  form: UseFormReturn<DownloadFormData>;
  handleKeyPress: (e: React.KeyboardEvent) => void;
}

export default function AdvancedTab({
  form,
  handleKeyPress,
}: AdvancedTabProps) {
  return (
    <TabsContent value="advanced" className="space-y-4 overflow-scroll p-2">
      <FormField
        control={form.control}
        name="filePath"
        render={({ field }) => (
          <FormItem className="gap-1 flex-col">
            <FormLabel htmlFor="filePath">File path</FormLabel>
            <FormControl>
              <Input
                {...field}
                placeholder={
                  process.platform === "win32"
                    ? "e.g. C:\\Users\\YourName\\Downloads"
                    : "e.g. /Users/yourname/Downloads"
                }
                onKeyPress={handleKeyPress}
                autoFocus
              />
            </FormControl>
            <div className="min-h-[20px]">
              <FormMessage />
            </div>
          </FormItem>
        )}
      />

      <SpeedLimitField form={form} handleKeyPress={handleKeyPress} />
      <PositiveNumberField
        form={form}
        name="maxRetries"
        label="Max Retries"
        handleKeyPress={handleKeyPress}
      />
      <PositiveNumberField
        form={form}
        name="backoffFactor"
        label="Backoff Factor"
        handleKeyPress={handleKeyPress}
      />
      <PositiveNumberField
        form={form}
        name="timeoutSecs"
        label="Timeout Duration (secs)"
        handleKeyPress={handleKeyPress}
      />
    </TabsContent>
  );
}
