import { useEffect } from 'react';
import { useFormContext } from 'react-hook-form';

import { FormControl, FormField, FormItem, FormLabel } from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';

import FormMessage from './form-message';

export default function ProxyField() {
  const { control, watch, setValue } = useFormContext();

  const proxyType: string = watch('proxy.type') ?? 'none';

  useEffect(() => {
    if (proxyType === 'none' || proxyType === 'system') {
      setValue('proxy.host', undefined, {
        shouldValidate: true,
        shouldDirty: true,
      });
      setValue('proxy.port', undefined, {
        shouldValidate: true,
        shouldDirty: true,
      });
      setValue('proxy.auth.username', undefined, {
        shouldValidate: true,
        shouldDirty: true,
      });
      setValue('proxy.auth.password', undefined, {
        shouldValidate: true,
        shouldDirty: true,
      });
    }
  }, [proxyType]);

  return (
    <div className="space-y-4 rounded-lg border p-2">
      <FormField
        control={control}
        name="proxy.type"
        render={({ field }) => (
          <FormItem className="flex items-center justify-between">
            <FormLabel>Proxy</FormLabel>
            <Select onValueChange={field.onChange} value={field.value ?? 'none'}>
              <FormControl>
                <SelectTrigger className="min-w-44">
                  <SelectValue placeholder="Select proxy type" />
                </SelectTrigger>
              </FormControl>
              <SelectContent>
                <SelectItem value="system">System</SelectItem>
                <SelectItem value="none">No Proxy</SelectItem>
                <SelectItem value="http">HTTP</SelectItem>
                <SelectItem value="https">HTTPS</SelectItem>
                <SelectItem value="socks5">SOCKS5</SelectItem>
              </SelectContent>
            </Select>
          </FormItem>
        )}
      />
      {proxyType !== 'none' && proxyType !== 'system' && (
        <div className="space-y-2">
          <div className="flex items-end gap-2">
            <FormField
              control={control}
              name="proxy.host"
              render={({ field }) => (
                <FormItem className="grow">
                  <FormLabel>Host</FormLabel>
                  <FormControl>
                    <Input placeholder="proxy.example.com" {...field} value={field.value ?? ''} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <div className="text-muted-foreground flex h-19 items-center px-1" aria-hidden>
              :
            </div>
            <FormField
              control={control}
              name="proxy.port"
              render={({ field }) => (
                <FormItem className="w-20">
                  <FormLabel>Port</FormLabel>
                  <FormControl>
                    <Input
                      type="number"
                      inputMode="numeric"
                      placeholder="8080"
                      value={field.value ?? ''}
                      onChange={(e) => {
                        const v = e.target.value;
                        field.onChange(v === '' ? undefined : Number(v));
                      }}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>

          <div className="grid grid-cols-2 gap-4">
            <FormField
              control={control}
              name="proxy.auth.username"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Username</FormLabel>
                  <FormControl>
                    <Input placeholder="username" {...field} value={field.value ?? ''} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={control}
              name="proxy.auth.password"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Password</FormLabel>
                  <FormControl>
                    <Input
                      type="password"
                      placeholder="••••••"
                      {...field}
                      value={field.value ?? ''}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>
        </div>
      )}
    </div>
  );
}
