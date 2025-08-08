"use client";

import { FormControl, FormField, FormItem, FormLabel } from "../ui/form";
import { Slider } from "../ui/slider";
import { useFormContext, UseFormReturn } from "react-hook-form";
import { DownloadFormData } from "./download-setting-sheet";
import FormMessage from "./form-message";

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
        const v = typeof field.value === "number" ? field.value : defaultValue;
        return (
          <FormItem className="gap-1 flex-col">
            <div className="flex items-center justify-between">
              <FormLabel htmlFor={String(name)}>{label}</FormLabel>
              <span className="text-xs text-muted-foreground">{v}</span>
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
