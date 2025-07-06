"use client";

import type React from "react";

import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { DownloadIcon, MoreVerticalIcon } from "lucide-react";
import { DownloadType } from "./types";
import { useForm } from "react-hook-form";

import { Form } from "./ui/form";

import { urlFormSchema } from "@/lib/validation";
import { z } from "zod";
import { FormControl, FormField, FormItem, FormMessage } from "./ui/form";
import { zodResolver } from "@hookform/resolvers/zod";
interface DownloadBarProps {
  setIsModalOpen: (open: boolean) => void;
  selectedMimeType: string | null;
  filteredDownloads: DownloadType[];
  url: string;
  setUrl: (url: string) => void;
}
import { Loading } from "./ui/loading";

type UrlFormData = z.infer<typeof urlFormSchema>;

export default function DownloadBar({
  selectedMimeType,
  setIsModalOpen,
  filteredDownloads,
  setUrl,
  url,
}: DownloadBarProps) {
  const [isLoading, setIsLoading] = useState(false);

  const form = useForm<UrlFormData>({
    resolver: zodResolver(urlFormSchema),
    defaultValues: { url },
  });

  const handleSubmit = async (value: z.infer<typeof urlFormSchema>) => {
    if (!value.url.trim()) return;

    setIsLoading(true);
    try {
      await invoke("add_new_download", {
        url: value.url,
        options: {
          chunk_count: 0
        }
      });
      form.setValue("url", "");
    } catch (error) {
      console.error("Failed to add download:", error);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="flex items-start gap-2 justify-between t-pb-20">
      <Form {...form}>
        <form
          className="flex gap-2 w-full"
          onSubmit={form.handleSubmit(handleSubmit)}
        >
          <FormField
            control={form.control}
            name="url"
            render={({ field }) => (
              <FormItem className="w-full gap-1">
                <FormControl>
                  <Input
                    placeholder="Enter a link to download."
                    {...field}
                    onChange={(ev) => {
                      setUrl(ev.target.value);
                      field.onChange(ev);
                    }}
                    className="w-full"
                    autoFocus
                  />
                </FormControl>
                <div className="min-h-[20px]">
                  <FormMessage />
                </div>
              </FormItem>
            )}
          />
          <Button
            type="submit"
            disabled={isLoading}
            className="flex items-center gap-2"
          >
            {isLoading ? (
              <Loading className="w-4 h-4" />
            ) : (
              <DownloadIcon className="w-4 h-4" />
            )}
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
        className="flex items-center gap-2"
      >
        <MoreVerticalIcon className="w-4 h-4" />
        Advanced
      </Button>
    </div>
  );
}
