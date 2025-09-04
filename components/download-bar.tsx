'use client';

import { zodResolver } from '@hookform/resolvers/zod';
import { invoke } from '@tauri-apps/api/core';
import { DownloadIcon, MoreHorizontal } from 'lucide-react';
import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { toast } from 'sonner';

interface DownloadBarProps {
  setIsModalOpen: (open: boolean) => void;
  url: string;
  setUrl: (url: string) => void;
}

import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { cn } from '@/lib/utils';
import { urlFormSchema } from '@/lib/validation';

import { Form, FormControl, FormField, FormItem } from './ui/form';
import { Loading } from './ui/loading';

import type React from 'react';
import type { z } from 'zod';

type UrlFormData = z.infer<typeof urlFormSchema>;

export default function DownloadBar({ setIsModalOpen, setUrl, url }: DownloadBarProps) {
  const [isLoading, setIsLoading] = useState(false);

  const form = useForm<UrlFormData>({
    resolver: zodResolver(urlFormSchema),
    defaultValues: { url },
    mode: 'onChange',
  });

  const urlError = form.formState.errors.url;

  const handleSubmit = async (value: z.infer<typeof urlFormSchema>) => {
    if (!value.url.trim()) return;

    setIsLoading(true);
    try {
      await invoke('add_new_download', {
        url: value.url,
        options: {
          chunk_count: 5,
        },
      });
      form.setValue('url', '');
      setUrl('');
    } catch (error) {
      toast.error(`${error}`);
    } finally {
      setIsLoading(false);
    }
  };

  const handleUrlFieldChange = (ev: React.ChangeEvent<HTMLInputElement>) => {
    if (urlError) {
      form.clearErrors('url');
    }
    setUrl(ev.target.value);
    form.setValue('url', ev.target.value, { shouldValidate: false });
  };

  return (
    <div
      className={cn(
        'bg-card flex items-start justify-between gap-2 border-b border-b-transparent py-1 pr-3 pl-1',
        {
          'border-b border-b-red-600/30': urlError,
        },
      )}
    >
      <Form {...form}>
        <form
          className="flex w-full gap-2"
          onSubmit={form.handleSubmit(handleSubmit, (errors) => {
            if (errors.url?.message) {
              toast.error(errors.url.message);
            }
          })}
        >
          <FormField
            control={form.control}
            name="url"
            render={({ field }) => (
              <FormItem className="w-full gap-1">
                <FormControl>
                  <Input
                    inputMode="url"
                    placeholder="Enter a link to download."
                    {...field}
                    onChange={handleUrlFieldChange}
                    className={cn(
                      'w-full border-none !bg-inherit shadow-none focus-visible:ring-0',
                    )}
                  />
                </FormControl>
              </FormItem>
            )}
          />
          <Button variant="ghost" type="submit" disabled={isLoading} className="flex items-center">
            {isLoading ? <Loading className="h-4 w-4" /> : <DownloadIcon className="h-4 w-4" />}
          </Button>
        </form>
      </Form>
      {/* <p className="text-muted-foreground">
          {selectedMimeType
            ? `Showing ${filteredDownloads.length} filtered downloads`
            : ""}
        </p> */}
      <Button
        onClick={() => setIsModalOpen(true)}
        className="flex items-center gap-1"
        variant="ghost"
        autoCorrect="off"
        spellCheck={false}
        autoCapitalize="none"
      >
        <MoreHorizontal className="h-4 w-4" />
      </Button>
    </div>
  );
}
