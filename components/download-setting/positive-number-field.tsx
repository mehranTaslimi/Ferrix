import { FormControl, FormField, FormItem, FormLabel } from "../ui/form";
import { Slider } from "../ui/slider";
import { UseFormReturn } from "react-hook-form";
import { DownloadFormData } from "./download-setting-sheet";
import FormMessage from "./form-message";

interface PositiveNumberFieldProps {
  form: UseFormReturn<DownloadFormData>;
  name: keyof DownloadFormData;
  label: string;
  min?: number;
  max?: number;
  defaultValue?: number;
  step?: number;
}

export default function PositiveNumberField({
  form,
  name,
  label,
  min = 1,
  max = 100,
  defaultValue = min,
  step = 1,
}: PositiveNumberFieldProps) {
  return (
    <FormField
      control={form.control}
      name={name}
      render={({ field }) => {
        const value =
          typeof field.value === "number" ? field.value : defaultValue;

        return (
          <FormItem className="gap-2 flex-col">
            <FormLabel htmlFor={name}>
              {label}: {value}
            </FormLabel>
            <FormControl>
              <Slider
                min={min}
                max={max}
                step={step}
                defaultValue={[defaultValue]}
                onValueChange={(value) => field.onChange(value[0])}
                value={[value]}
              />
            </FormControl>
          </FormItem>
        );
      }}
    />
  );
}
