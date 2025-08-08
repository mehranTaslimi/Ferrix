"use client";

import type React from "react";

import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { DownloadIcon, MoreHorizontal } from "lucide-react";
import { DownloadType } from "./types";
import { useForm } from "react-hook-form";

import { Form } from "./ui/form";

import { urlFormSchema } from "@/lib/validation";
import { z } from "zod";
import { FormControl, FormField, FormItem } from "./ui/form";
import { zodResolver } from "@hookform/resolvers/zod";
interface DownloadBarProps {
  setIsModalOpen: (open: boolean) => void;
  url: string;
  setUrl: (url: string) => void;
}
import { Loading } from "./ui/loading";
import { toast } from "sonner";
import { cn } from "@/lib/utils";

type UrlFormData = z.infer<typeof urlFormSchema>;

export default function DownloadBar({
  setIsModalOpen,
  setUrl,
  url,
}: DownloadBarProps) {
  const [isLoading, setIsLoading] = useState(false);

  const form = useForm<UrlFormData>({
    resolver: zodResolver(urlFormSchema),
    defaultValues: { url },
    mode: "onChange",
  });

  const urlError = form.formState.errors.url;

  const handleSubmit = async (value: z.infer<typeof urlFormSchema>) => {
    if (!value.url.trim()) return;

    setIsLoading(true);
    try {
      await invoke("add_new_download", {
        url: value.url,
        options: {
          chunk_count: 5,
        },
      });
      form.setValue("url", "");
    } catch (error) {
      console.error("Failed to add download:", error);
      toast.error("Failed to add download. Please check the URL.");
    } finally {
      setIsLoading(false);
    }
  };

  const handleUrlFieldChange = (ev: React.ChangeEvent<HTMLInputElement>) => {
    if (urlError) {
      form.clearErrors("url");
    }
    setUrl(ev.target.value);
    form.setValue("url", ev.target.value, { shouldValidate: false });
  };

  return (
    <div
      className={cn(
        "flex items-start gap-2 justify-between py-1 pl-1 pr-3 bg-card border-b border-b-transparent",
        {
          "border-b border-b-red-600/30": urlError,
        }
      )}
    >
      <Form {...form}>
        <form
          className="flex gap-2 w-full"
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
                      "w-full border-none !bg-inherit focus-visible:ring-0 shadow-none"
                    )}
                  />
                </FormControl>
              </FormItem>
            )}
          />
          <Button
            variant="ghost"
            type="submit"
            disabled={isLoading}
            className="flex items-center"
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
        className="flex items-center gap-1"
        variant="ghost"
        autoCorrect="off"
        spellCheck={false}
        autoCapitalize="none"
      >
        <MoreHorizontal className="w-4 h-4" />
      </Button>
    </div>
  );
}
