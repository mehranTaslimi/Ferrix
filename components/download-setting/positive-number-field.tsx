'use client';

import { useFormContext } from 'react-hook-form';

import { FormControl, FormField, FormItem, FormLabel } from '../ui/form';
import { Slider } from '../ui/slider';

import FormMessage from './form-message';

import type { DownloadFormData } from './download-setting-sheet';

interface PositiveNumberFieldProps {
  name: keyof DownloadFormData;
  label: string;
  min?: number;
  max?: number;
  defaultValue?: number;
  step?: number;
}

export default function PositiveNumberField({
  name,
  label,
  min = 1,
  max = 100,
  defaultValue = min,
  step = 1,
}: PositiveNumberFieldProps) {
  const { control } = useFormContext();

  return (
    <FormField
      control={control}
      name={name}
      render={({ field }) => {
        const v = typeof field.value === 'number' ? field.value : defaultValue;
        return (
          <FormItem className="flex-col gap-1">
            <div className="flex items-center justify-between">
              <FormLabel htmlFor={String(name)}>{label}</FormLabel>
              <span className="text-muted-foreground text-xs">{v}</span>
            </div>
            <FormControl>
              <Slider
                min={min}
                max={max}
                step={step}
                value={[v]}
                onValueChange={(value) => field.onChange(value[0])}
              />
            </FormControl>
            <FormMessage />
          </FormItem>
        );
      }}
    />
  );
}
