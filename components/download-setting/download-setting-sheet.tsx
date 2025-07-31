"use client";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Plus } from "lucide-react";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";

import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
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

export interface TabsValue {
  Basic: "basic";
  Advanced: "advanced";
}

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
    },
  });

  const handleSubmit = async (values: DownloadFormData) => {
    if (!values.url.trim()) return;

    setIsLoading(true);
    try {
      await invoke("add_new_download", {
        url: values.url.trim(),
        options: {
          chunk_count: values.chunk,
          file_path: values.filePath,
          speed_limit: values.speedLimit,
          max_retries: values.maxRetries,
          backoff_factor: values.backoffFactor,
          timeout_secs: values.timeoutSecs,
        },
      });
      setUrl(url);
      onOpenChange(false);
      form.reset({ url, chunk: 5 });
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
          <SheetTitle>Download Setting</SheetTitle>
          <SheetDescription>
            Enter a URL and configure download settings to start downloading a
            file.
          </SheetDescription>
        </SheetHeader>

        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(handleSubmit)}
            className="h-full flex flex-col justify-between overflow-y-auto"
          >
            <Tabs className="overflow-y-auto">
              <TabsList className="grid w-full grid-cols-2">
                <TabsTrigger value="basic">Basic Settings</TabsTrigger>
                <TabsTrigger value="advanced">Advanced Settings</TabsTrigger>
              </TabsList>
              <BasicTab form={form} handleKeyPress={handleKeyPress} />
              <AdvancedTab form={form} handleKeyPress={handleKeyPress} />
            </Tabs>

            <div className="flex gap-2 pt-2">
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
                {isLoading ? <Loading /> : "Add Download"}
              </Button>
            </div>
          </form>
        </Form>
      </SheetContent>
    </Sheet>
  );
}
