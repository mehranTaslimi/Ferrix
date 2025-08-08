"use client";

import { TabsContent } from "@radix-ui/react-tabs";
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
} from "../ui/form";
import { Input } from "../ui/input";
import { useFormContext } from "react-hook-form";
import FormMessage from "./form-message";
import { Slider } from "../ui/slider";

interface BasicTabProps {
  handleKeyPress: (e: React.KeyboardEvent) => void;
}
export default function BasicTab({ handleKeyPress }: BasicTabProps) {
  const { control } = useFormContext();
  return (
    <TabsContent value="basic" className="p-2 space-y-3 overflow-y-scroll">
      <FormField
        control={control}
        name="url"
        render={({ field }) => (
          <FormItem className="gap-1 flex-col">
            <FormLabel htmlFor="url">Download URL *</FormLabel>
            <FormControl>
              <Input
                {...field}
                inputMode="url"
                placeholder="https://example.com/file.mp4"
                onKeyDown={handleKeyPress}
                autoFocus
              />
            </FormControl>
            <FormMessage />
          </FormItem>
        )}
      />

      <FormField
        control={control}
        name="chunk"
        render={({ field }) => {
          const val = typeof field.value === "number" ? field.value : 5;
          return (
            <FormItem className="gap-1 flex-col">
              <div className="flex items-center justify-between">
                <FormLabel htmlFor="chunk">Number of chunks</FormLabel>
                <span className="text-xs text-muted-foreground">{val}</span>
              </div>
              <FormControl>
                <Slider
                  min={1}
                  max={16}
                  step={1}
                  value={[val]}
                  onValueChange={(v) => field.onChange(v[0])}
                />
              </FormControl>
              <FormDescription>
                More chunks can improve speed but use more resources (1–16).
              </FormDescription>
              <FormMessage />
            </FormItem>
          );
        }}
      />
    </TabsContent>
  );
}
