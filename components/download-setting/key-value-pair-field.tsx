import { useFieldArray, UseFormReturn } from "react-hook-form";
import { Button } from "../ui/button";
import { Input } from "../ui/input";
import { FormControl, FormField, FormItem } from "../ui/form";
import { Plus, Trash2 } from "lucide-react";
import { DownloadFormData } from "./download-setting-sheet";
import FormMessage from "./form-message";

interface KeyValuePairFieldProps {
  form: UseFormReturn<DownloadFormData>;
  label: string;
  name: "headers" | "cookies";
  handleKeyPress?: (e: React.KeyboardEvent) => void;
}

export default function KeyValuePairField({
  form,
  label,
  name,
  handleKeyPress,
}: KeyValuePairFieldProps) {
  const { fields, append, remove } = useFieldArray({
    control: form.control,
    name,
  });

  return (
    <div className="space-y-1">
      {label && <label className="text-sm font-medium">{label}</label>}
      <div className="space-y-2">
        {fields.map((field, index) => (
          <div key={field.id} className="flex gap-2 items-start">
            <FormField
              control={form.control}
              name={`${name}.${index}.key`}
              render={({ field }) => (
                <FormItem className="flex-1 gap-1">
                  <FormControl>
                    <Input
                      {...field}
                      placeholder={`${label} name`}
                      onKeyDown={handleKeyPress}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name={`${name}.${index}.value`}
              render={({ field }) => (
                <FormItem className="flex-1 gap-1">
                  <FormControl>
                    <Input
                      {...field}
                      placeholder={`${label} value`}
                      onKeyDown={handleKeyPress}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <Button
              type="button"
              variant="outline"
              size="icon"
              onClick={() => remove(index)}
            >
              <Trash2 className="w-4 h-4" />
            </Button>
          </div>
        ))}
        <Button
          type="button"
          variant="outline"
          className="w-full"
          onClick={() => append({ key: "", value: "" })}
        >
          <Plus className="w-4 h-4 mr-2" />
          Add {label}
        </Button>
      </div>
    </div>
  );
}
