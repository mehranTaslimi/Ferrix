'use client';

import { TabsContent } from '@radix-ui/react-tabs';
import { useFormContext } from 'react-hook-form';

import { FormControl, FormDescription, FormField, FormItem, FormLabel } from '../ui/form';
import { Input } from '../ui/input';
import { Slider } from '../ui/slider';

import FormMessage from './form-message';

interface BasicTabProps {
  handleKeyPress: (e: React.KeyboardEvent) => void;
}
export default function BasicTab({ handleKeyPress }: BasicTabProps) {
  const { control } = useFormContext();
  return (
    <TabsContent value="basic" className="space-y-3 p-2">
      <FormField
        control={control}
        name="url"
        render={({ field }) => (
          <FormItem className="flex-col gap-1">
            <FormLabel htmlFor="url">Download URL *</FormLabel>
            <FormControl>
              <Input
                {...field}
                inputMode="url"
                placeholder="https://example.com/file.mp4"
                onKeyDown={handleKeyPress}
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
          const val = typeof field.value === 'number' ? field.value : 5;
          return (
            <FormItem className="flex-col gap-1">
              <div className="flex items-center justify-between">
                <FormLabel htmlFor="chunk">Number of chunks</FormLabel>
                <span className="text-muted-foreground text-xs">{val}</span>
              </div>
              <FormControl>
                <Slider
                  min={1}
                  max={5}
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
