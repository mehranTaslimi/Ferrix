"use client";

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";

import { Form } from "@/components/ui/form";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { Tabs, TabsList, TabsTrigger } from "../ui/tabs";
import BasicTab from "./basic-tab";
import AdvancedTab from "./advanced-tab";
import { downloadFormSchema } from "@/lib/validation";
import { toast } from "sonner";
import { Loading } from "../ui/loading";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
} from "../ui/sheet";

export type DownloadFormData = z.infer<typeof downloadFormSchema>;

interface DownloadSettingSheetProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  url: string;
  setUrl: (url: string) => void;
}

export default function DownloadSettingSheet({
  open,
  onOpenChange,
  setUrl,
  url,
}: DownloadSettingSheetProps) {
  const [isLoading, setIsLoading] = useState(false);

  const form = useForm<DownloadFormData>({
    resolver: zodResolver(downloadFormSchema),
    defaultValues: {
      url,
      chunk: 5,
      headers: [],
      cookies: [],
      proxy: {
        enabled: false,
        type: "http",
        host: "",
        port: 8080,
        auth: undefined,
      },
    },
    mode: "onBlur",
  });

  useEffect(() => {
    if (open) {
      // sync incoming URL into form on open
      form.reset({ url, chunk: form.getValues("chunk") || 5 });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [open]);

  const handleSubmit = async (values: DownloadFormData) => {
    if (!values.url?.trim()) {
      form.setError("url", { type: "manual", message: "URL is required" });
      return;
    }

    const kvToRecord = (arr?: Array<{ key: string; value: string }>) =>
      (arr || []).reduce<Record<string, string>>((acc, { key, value }) => {
        if (key && value) acc[key] = value;
        return acc;
      }, {});

    const proxy = values.proxy?.enabled
      ? {
          type: values.proxy.type,
          host: values.proxy.host,
          port: values.proxy.port,
          username: values.proxy?.auth?.username,
          password: values.proxy?.auth?.password,
        }
      : undefined;
    const headers = kvToRecord(values.headers);
    const cookies = kvToRecord(values.cookies);

    setIsLoading(true);
    try {
      await invoke("add_new_download", {
        url: values.url.trim(),
        options: {
          proxy,
          ...(values.auth && {
            auth: { ...values.auth, type: values.auth.type.toLowerCase() },
          }),
          headers: Object.keys(headers).length ? headers : undefined,
          cookies: Object.keys(cookies).length ? cookies : undefined,
          chunk_count: values.chunk,
          file_path: values.filePath || undefined,
          speed_limit: values.speedLimit || undefined,
          max_retries: values.maxRetries || undefined,
          backoff_factor: values.backoffFactor || undefined,
          timeout_secs: values.timeoutSecs || undefined,
        },
      });

      toast.success("Download added");
      setUrl(values.url.trim());
      onOpenChange(false);
      form.reset({
        url: values.url.trim(),
        chunk: 5,
        headers: [],
        cookies: [],
      });
    } catch (err: any) {
      console.error("Failed to add download:", err);
      toast.error("Failed to add download", {
        description:
          typeof err?.message === "string"
            ? err.message
            : "Please check the URL or settings and try again.",
      });
    } finally {
      setIsLoading(false);
    }
  };

  const onKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      form.handleSubmit(handleSubmit)();
    }
  };

  return (
    <Sheet open={open} onOpenChange={(v) => !isLoading && onOpenChange(v)}>
      <SheetContent className="sm:max-w-md p-2 h-full">
        <SheetHeader className="px-1">
          <SheetTitle>New download</SheetTitle>
          <SheetDescription>Paste a URL, tweak options, done.</SheetDescription>
        </SheetHeader>

        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(handleSubmit)}
            className="h-full flex flex-col gap-2"
          >
            <Tabs defaultValue="basic" className="flex-1 overflow-hidden">
              <TabsList className="grid w-full grid-cols-2 sticky top-0 z-10">
                <TabsTrigger value="basic">Basic</TabsTrigger>
                <TabsTrigger value="advanced">Advanced</TabsTrigger>
              </TabsList>

              <BasicTab handleKeyPress={onKeyPress} />

              <AdvancedTab handleKeyPress={onKeyPress} />
            </Tabs>

            <div className="flex gap-2 pt-4 pb-2 sticky bottom-0 bg-background">
              <Button
                variant="outline"
                onClick={() => onOpenChange(false)}
                disabled={isLoading}
                className="flex-1"
                type="button"
              >
                Cancel
              </Button>
              <Button type="submit" disabled={isLoading} className="flex-1">
                {isLoading ? <Loading /> : "Start Download"}
              </Button>
            </div>
          </form>
        </Form>
      </SheetContent>
    </Sheet>
  );
}
