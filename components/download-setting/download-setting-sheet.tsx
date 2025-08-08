"use client";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Plus } from "lucide-react";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";

import { Form } from "@/components/ui/form";
import { downloadFormSchema } from "@/lib/validation";
import { Loading } from "../ui/loading";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
} from "../ui/sheet";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "../ui/tabs";
import BasicTab from "./basic-tab";
import AdvancedTab from "./advanced-tab";

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
  });

  const handleSubmit = async (values: DownloadFormData) => {
    if (!values.url.trim()) return;

    const arrayToRecord = (values?: Array<{ value: string; key: string }>) =>
      values?.reduce((acc, { key, value }) => {
        if (key && value) acc[key] = value;
        return acc;
      }, {} as Record<string, string>);

    const headers = arrayToRecord(values.headers);
    const cookies = arrayToRecord(values.cookies);
    const proxy = values.proxy?.enabled
      ? {
          type: values.proxy.type,
          host: values.proxy.host,
          port: values.proxy.port,
          username: values.proxy?.auth?.username,
          password: values.proxy?.auth?.password,
        }
      : undefined;

    setIsLoading(true);
    try {
      await invoke("add_new_download", {
        url: values.url.trim(),
        options: {
          headers: Object.keys(headers || {}).length > 0 ? headers : undefined,
          cookies: Object.keys(cookies || {}).length > 0 ? cookies : undefined,
          proxy,
          ...(values.auth && {
            auth: { ...values.auth, type: values.auth.type.toLowerCase() },
          }),
          chunk_count: values.chunk,
          file_path: values.filePath,
          speed_limit: values.speedLimit,
          max_retries: values.maxRetries,
          backoff_factor: values.backoffFactor,
          timeout_secs: values.timeoutSecs,
        },
      });
      setUrl(values.url.trim());
      onOpenChange(false);
      form.reset({
        url: values.url.trim(),
        chunk: 5,
        headers: [],
        cookies: [],
      });
    } catch (error) {
      console.error("Failed to add download:", error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      form.handleSubmit(handleSubmit)();
    }
  };

  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === "Escape") onOpenChange(false);
    };
    if (open) {
      document.addEventListener("keydown", handleEscape);
      return () => document.removeEventListener("keydown", handleEscape);
    }
  }, [open, onOpenChange]);

  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <SheetContent className="sm:max-w-md p-2 h-full">
        <SheetHeader>
          <SheetTitle>Download Settings</SheetTitle>
          <SheetDescription>
            Configure download parameters including proxy settings
          </SheetDescription>
        </SheetHeader>

        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(handleSubmit)}
            className="h-full flex flex-col justify-between overflow-y-auto"
          >
            <Tabs defaultValue="basic" className="overflow-y-auto">
              <TabsList className="grid w-full grid-cols-2">
                <TabsTrigger value="basic">Basic</TabsTrigger>
                <TabsTrigger value="advanced">Advanced</TabsTrigger>
              </TabsList>

              <TabsContent value="basic" className="space-y-4">
                <BasicTab handleKeyPress={handleKeyPress} />
              </TabsContent>

              <TabsContent value="advanced" className="space-y-4">
                <AdvancedTab handleKeyPress={handleKeyPress} />
              </TabsContent>
            </Tabs>

            <div className="flex gap-2 pt-4 sticky bottom-0 bg-background">
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
