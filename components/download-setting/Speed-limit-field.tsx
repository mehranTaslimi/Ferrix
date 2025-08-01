import {
  FormField,
  FormItem,
  FormLabel,
  FormControl,
  FormDescription,
  FormMessage,
} from "../ui/form";
import { Input } from "../ui/input";
import { UseFormReturn } from "react-hook-form";
import { DownloadFormData } from "./download-setting-sheet";
import { useEffect, useState } from "react";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "../ui/select";

const UNIT_MULTIPLIERS = {
  KB: 1024,
  MB: 1024 * 1024,
  GB: 1024 * 1024 * 1024,
} as const;

type Unit = keyof typeof UNIT_MULTIPLIERS;

interface SpeedLimitFieldProps {
  form: UseFormReturn<DownloadFormData>;
  handleKeyPress?: (e: React.KeyboardEvent) => void;
}

export default function SpeedLimitField({
  form,
  handleKeyPress,
}: SpeedLimitFieldProps) {
  const fieldName: keyof DownloadFormData = "speedLimit";
  const [unit, setUnit] = useState<Unit>("KB");
  const byteValue = form.watch(fieldName);

  const displayValue = byteValue
    ? Math.floor(byteValue / UNIT_MULTIPLIERS[unit])
    : "";

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const numericValue = parseFloat(e.target.value);
    if (!isNaN(numericValue)) {
      const inBytes = numericValue * UNIT_MULTIPLIERS[unit];
      form.setValue(fieldName, Math.floor(inBytes));
    } else {
      form.setValue(fieldName, 0);
    }
  };

  useEffect(() => {
    if (byteValue) {
      const newByteValue = Math.floor(
        Math.floor(byteValue / UNIT_MULTIPLIERS[unit]) * UNIT_MULTIPLIERS[unit]
      );
      form.setValue(fieldName, newByteValue);
    }
  }, [unit, byteValue]);

  return (
    <FormField
      control={form.control}
      name={fieldName}
      render={() => (
        <FormItem className="gap-1 flex-col">
          <FormLabel htmlFor={fieldName}>Speed Limit</FormLabel>
          <div className="flex items-center gap-2">
            <FormControl>
              <Input
                type="number"
                value={displayValue}
                onChange={handleChange}
                min={1}
              />
            </FormControl>
            <Select
              value={unit}
              onValueChange={(value) => setUnit(value as Unit)}
            >
              <SelectTrigger>
                <SelectValue placeholder="Unit" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="KB">KB</SelectItem>
                <SelectItem value="MB">MB</SelectItem>
                <SelectItem value="GB">GB</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div className="min-h-[20px]">
            <FormMessage />
          </div>
        </FormItem>
      )}
    />
  );
}
