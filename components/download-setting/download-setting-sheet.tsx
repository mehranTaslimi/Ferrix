"use client";

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";

import { Form } from "@/components/ui/form";
import { Button } from "@/components/ui/button";
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
const getDefaultFormValues = (url: string): DownloadFormData => ({
  maxRetries: 3,
  backoffFactor: 2,
  timeoutSecs: 30,
  url,
  chunk: 5,
  headers: [],
  cookies: [],
  auth: { type: "None" },
  proxy: { type: "system" },
  filePath: "",
});

export default function DownloadSettingSheet({
  open,
  onOpenChange,
  setUrl,
  url,
}: DownloadSettingSheetProps) {
  const [isLoading, setIsLoading] = useState(false);
  const form = useForm<DownloadFormData>({
    resolver: zodResolver(downloadFormSchema),
    defaultValues: getDefaultFormValues(url),
    mode: "onBlur",
  });

  useEffect(() => {
    if (open) {
      form.setValue("url", url.trim());
    }
  }, [open]);

  const handleSubmit = async (values: DownloadFormData) => {
    console.log("Submitting form with values:", values);
    if (!values.url?.trim()) {
      form.setError("url", { type: "manual", message: "URL is required" });
      return;
    }

    const kvToRecord = (arr?: Array<{ key: string; value: string }>) =>
      (arr || []).reduce<Record<string, string>>((acc, { key, value }) => {
        if (key && value) acc[key] = value;
        return acc;
      }, {});

    const proxy =
      values.proxy?.type !== "none" && values.proxy?.type !== "system"
        ? {
            type: values.proxy.type.toLowerCase(),
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
          ...(values.auth &&
            values.auth?.type !== "None" && {
              auth: { ...values.auth, type: values.auth.type.toLowerCase() },
            }),
          headers: Object.keys(headers).length ? headers : undefined,
          cookies: Object.keys(cookies).length ? cookies : undefined,
          chunk_count: values.chunk,
          ...(!!values.filePath && {
            file_path: values.filePath,
          }),
          speed_limit: values.speedLimit || undefined,
          max_retries: values.maxRetries || undefined,
          backoff_factor: values.backoffFactor || undefined,
          timeout_secs: values.timeoutSecs || undefined,
        },
      });

      toast.success("Download added");
      onOpenChange(false);
      setUrl("");
      form.reset(getDefaultFormValues(""));
    } catch (err: unknown) {
      const error = err as Error;
      console.error("Failed to add download:", err);
      toast.error("Failed to add download", {
        description:
          typeof error?.message === "string"
            ? error.message
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
            className="h-full flex flex-col gap-2 overflow-y-auto"
          >
            <Tabs defaultValue="basic" className="flex-1">
              <TabsList className="grid w-full grid-cols-2 sticky top-0 z-10 dark:bg-neutral-900 bg-neutral-200">
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
