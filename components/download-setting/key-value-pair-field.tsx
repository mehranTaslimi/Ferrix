import { Plus, Trash2 } from 'lucide-react';
import { useFieldArray, useFormContext } from 'react-hook-form';

import { Button } from '../ui/button';
import { FormControl, FormField, FormItem } from '../ui/form';
import { Input } from '../ui/input';

import FormMessage from './form-message';

interface KeyValuePairFieldProps {
  label: string;
  name: 'headers' | 'cookies';
  handleKeyPress?: (e: React.KeyboardEvent) => void;
}

export default function KeyValuePairField({ label, name, handleKeyPress }: KeyValuePairFieldProps) {
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
          <div className="text-muted-foreground text-xs">No {label.toLowerCase()} added.</div>
        )}

        {fields.map((f, index) => (
          <div key={f.id} className="flex items-start gap-2">
            <FormField
              control={control}
              name={`${name}.${index}.key`}
              render={({ field }) => (
                <FormItem className="flex-1 gap-1">
                  <FormControl>
                    <Input {...field} placeholder={`${label} name`} onKeyDown={handleKeyPress} />
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
                    <Input {...field} placeholder={`${label} value`} onKeyDown={handleKeyPress} />
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
              <Trash2 className="h-4 w-4" />
            </Button>
          </div>
        ))}

        <Button
          type="button"
          variant="outline"
          className="w-full"
          onClick={() => append({ key: '', value: '' })}
        >
          <Plus className="mr-2 h-4 w-4" />
          Add {label}
        </Button>
      </div>
    </div>
  );
}
