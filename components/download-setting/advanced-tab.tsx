'use client';

import { TabsContent } from '@radix-ui/react-tabs';
import { open } from '@tauri-apps/plugin-dialog';
import { platform as getPlatform } from '@tauri-apps/plugin-os';
import { FolderOpen } from 'lucide-react';
import { useEffect, useMemo, useState } from 'react';
import { useFormContext } from 'react-hook-form';

import { Button } from '../ui/button';
import { FormControl, FormField, FormItem, FormLabel } from '../ui/form';
import { Input } from '../ui/input';

import AuthField from './auth-field';
import FormMessage from './form-message';
import KeyValuePairField from './key-value-pair-field';
import PositiveNumberField from './positive-number-field';
import ProxyField from './proxy-field';

interface AdvancedTabProps {
  handleKeyPress: (e: React.KeyboardEvent) => void;
}

export default function AdvancedTab({ handleKeyPress }: AdvancedTabProps) {
  const form = useFormContext();
  const [os, setOs] = useState<'windows' | 'macos' | 'linux' | 'unknown'>('unknown');

  useEffect(() => {
    (async () => {
      try {
        const p = await getPlatform();
        if (p === 'windows' || p === 'macos' || p === 'linux') setOs(p);
      } catch {
        setOs('unknown');
      }
    })();
  }, []);

  const placeholder = useMemo(() => {
    if (os === 'windows') return 'e.g. C:\\Users\\YourName\\Downloads';
    if (os === 'macos') return 'e.g. /Users/yourname/Downloads';
    if (os === 'linux') return 'e.g. /home/yourname/Downloads';
    return 'Select a folder…';
  }, [os]);

  const handleSelectDirectory = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Select Download Directory',
      });
      if (typeof selected === 'string') {
        form.setValue('filePath', selected, {
          shouldValidate: true,
          shouldDirty: true,
        });
      } else if (Array.isArray(selected) && selected[0]) {
        form.setValue('filePath', selected[0], {
          shouldValidate: true,
          shouldDirty: true,
        });
      }
    } catch (error) {
      console.error('Error selecting directory:', error);
      form.setError('filePath', {
        type: 'manual',
        message: 'Failed to select directory. Please try again.',
      });
    }
  };

  return (
    <TabsContent value="advanced" className="space-y-3 p-2">
      <FormField
        control={form.control}
        name="filePath"
        render={({ field }) => (
          <FormItem className="flex-col gap-1">
            <FormLabel htmlFor="filePath">Download location</FormLabel>
            <div className="flex gap-2">
              <FormControl>
                <Input
                  id="filePath"
                  disabled
                  {...field}
                  placeholder={placeholder}
                  className="flex-1"
                />
              </FormControl>
              <Button
                type="button"
                variant="outline"
                onClick={handleSelectDirectory}
                className="shrink-0"
              >
                <FolderOpen className="mr-2 h-4 w-4" />
                Browse
              </Button>
            </div>
            <FormMessage />
          </FormItem>
        )}
      />
      <ProxyField />
      <AuthField />
      <KeyValuePairField name="headers" label="Headers" handleKeyPress={handleKeyPress} />
      <KeyValuePairField name="cookies" label="Cookies" handleKeyPress={handleKeyPress} />
      {/* <SpeedLimitField form={form} /> */}
      <PositiveNumberField
        max={30}
        defaultValue={3}
        min={0}
        name="maxRetries"
        label="Max Retries"
      />
      <PositiveNumberField
        min={2}
        defaultValue={2}
        max={10}
        name="backoffFactor"
        label="Backoff Factor"
      />
      <PositiveNumberField
        min={10}
        max={120}
        defaultValue={30}
        name="timeoutSecs"
        label="Timeout Duration (secs)"
      />
    </TabsContent>
  );
}
