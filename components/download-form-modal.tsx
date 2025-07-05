"use client";

import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "@/components/ui/dialog";
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
import { Loading } from "./ui/loading";

type DownloadFormData = z.infer<typeof downloadFormSchema>;

interface DownloadFormModalProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  url: string;
  setUrl: (url: string) => void;
}

export default function DownloadFormModal({
  open,
  onOpenChange,
  setUrl,
  url,
}: DownloadFormModalProps) {
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
      await invoke("add_download_queue", {
        url: values.url.trim(),
        chunk: values.chunk,
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
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <div className="flex items-center justify-between">
            <DialogTitle className="flex items-center gap-2">
              <Plus className="w-5 h-5" />
              Add New Download
            </DialogTitle>
          </div>
          <DialogDescription>
            Enter a URL and configure download settings to start downloading a
            file
          </DialogDescription>
        </DialogHeader>

        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(handleSubmit)}
            className="space-y-4 pt-2"
          >
            <FormField
              control={form.control}
              name="url"
              render={({ field }) => (
                <FormItem className="gap-1 flex-col">
                  <FormLabel htmlFor="url">Download URL *</FormLabel>
                  <FormControl>
                    <Input
                      {...field}
                      placeholder="https://example.com/file.mp4"
                      onKeyPress={handleKeyPress}
                      autoFocus
                    />
                  </FormControl>
                  <div className="min-h-[20px]">
                    <FormMessage />
                  </div>
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name="chunk"
              render={({ field }) => (
                <FormItem className="gap-1 flex-col">
                  <FormLabel htmlFor="chunk">Number of Chunks</FormLabel>
                  <FormControl>
                    <Input
                      {...field}
                      type="number"
                      min={1}
                      onKeyPress={handleKeyPress}
                      onChange={(e) => field.onChange(e.target.valueAsNumber)}
                    />
                  </FormControl>
                  <FormDescription>
                    Higher chunk count may increase download speed but uses more
                    resources (1-16)
                  </FormDescription>
                  <div className="min-h-[20px]">
                    <FormMessage />
                  </div>
                </FormItem>
              )}
            />

            <div className="flex gap-2 pt-4">
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
      </DialogContent>
    </Dialog>
  );
}
