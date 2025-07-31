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

interface PositiveNumberFieldProps {
  form: UseFormReturn<DownloadFormData>;
  name: keyof DownloadFormData;
  label: string;
  placeholder?: string;
  handleKeyPress?: (e: React.KeyboardEvent) => void;
}

export default function PositiveNumberField({
  form,
  name,
  label,
  placeholder,
  handleKeyPress,
}: PositiveNumberFieldProps) {
  return (
    <FormField
      control={form.control}
      name={name}
      render={({ field }) => (
        <FormItem className="gap-1 flex-col">
          <FormLabel htmlFor={name}>{label}</FormLabel>
          <FormControl>
            <Input
              {...field}
              type="number"
              placeholder={placeholder || `Enter ${label.toLowerCase()}`}
              min={1}
            />
          </FormControl>
          <div className="min-h-[20px]">
            <FormMessage />
          </div>
        </FormItem>
      )}
    />
  );
}
