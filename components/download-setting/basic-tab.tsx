import { TabsContent } from "@radix-ui/react-tabs";
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "../ui/form";
import { Input } from "../ui/input";
import { UseFormReturn } from "react-hook-form";
import { DownloadFormData } from "./download-setting-sheet";

interface BasicTabProps {
  form: UseFormReturn<DownloadFormData>;
  handleKeyPress: (e: React.KeyboardEvent) => void;
}
export default function BasicTab({ form, handleKeyPress }: BasicTabProps) {
  return (
    <TabsContent value="basic" className="p-2">
      <FormField
        control={form.control}
        name="url"
        render={({ field }) => (
          <FormItem className="gap-1 flex-col">
            <FormLabel htmlFor="url">Download URL *</FormLabel>
            <FormControl>
              <Input
                {...field}
                placeholder="https://example.com/file.mp4"
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
      <FormField
        control={form.control}
        name="chunk"
        render={({ field }) => (
          <FormItem className="gap-1 flex-col">
            <FormLabel htmlFor="chunk">Number of Chunks</FormLabel>
            <FormControl>
              <Input
                {...field}
                type="number"
                min={1}
                onKeyPress={handleKeyPress}
              />
            </FormControl>
            <FormDescription>
              Higher chunk count may increase download speed but uses more
              resources (1-16)
            </FormDescription>
            <div className="min-h-[20px]">
              <FormMessage />
            </div>
          </FormItem>
        )}
      />
    </TabsContent>
  );
}
