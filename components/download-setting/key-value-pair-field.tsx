import { useFieldArray, useFormContext } from "react-hook-form";
import { Button } from "../ui/button";
import { Input } from "../ui/input";
import { FormControl, FormField, FormItem } from "../ui/form";
import { Plus, Trash2 } from "lucide-react";
import FormMessage from "./form-message";

interface KeyValuePairFieldProps {
  label: string;
  name: "headers" | "cookies";
  handleKeyPress?: (e: React.KeyboardEvent) => void;
}

export default function KeyValuePairField({
  label,
  name,
  handleKeyPress,
}: KeyValuePairFieldProps) {
  const { control } = useFormContext();
  const { fields, append, remove } = useFieldArray({
    control,
    name,
  });

  return (
    <div className="space-y-2">
      <label className="text-sm font-medium">{label}</label>

      <div className="space-y-2">
        {fields.length === 0 && (
          <div className="text-xs text-muted-foreground">
            No {label.toLowerCase()} added.
          </div>
        )}

        {fields.map((f, index) => (
          <div key={f.id} className="flex gap-2 items-start">
            <FormField
              control={control}
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
              control={control}
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
              aria-label={`Remove ${label}`}
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
