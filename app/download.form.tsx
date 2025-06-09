"use client";

import { useState, useEffect } from 'react';
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
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';


export default function DownloadForm() {
  const form = useForm({
    defaultValues: {
      downloadUrl: "https://dl3.soft98.ir/win/Windows.11.v24H2.Build.26100.4061.x64-VL.part1.rar?1749473936",
    },
  });

  useEffect(() => {
    listen("process", (ev) => {
      console.log(ev);
    });
    listen("data", (ev) => {
      console.log(ev);
    });
    listen("download_list", (ev) => {
      console.log(ev);
    });
    (async () => {
      try {
        await invoke("get_download_list");
      } catch (e) {
        console.log(e);
      }
    })();
  }, [])

  async function onSubmitForm(values: { downloadUrl: string }) {
    try {
      await invoke("add_download_queue", {
        url: values.downloadUrl,
      });
    } catch (e) {
      console.log(e);
    }
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
