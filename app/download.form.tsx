"use client";

import { useState } from "react";
import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { useForm } from "react-hook-form";

interface DownloadFormProps {
  onSubmit: (url: string) => void;
}

export default function DownloadForm({ onSubmit }: DownloadFormProps) {
  const form = useForm({
    defaultValues: {
      downloadUrl: "",
    },
  });

  function onSubmitForm(values: { downloadUrl: string }) {
    onSubmit(values.downloadUrl);
  }

  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit(onSubmitForm)}
        className="space-y-6 w-full max-w-md"
      >
        <FormField
          control={form.control}
          name="downloadUrl"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Download URL</FormLabel>
              <FormControl>
                <Input placeholder="Enter download URL" {...field} />
              </FormControl>
            </FormItem>
          )}
        />

        <Button type="submit" className="w-full">
          Submit
        </Button>
      </form>
    </Form>
  );
}
