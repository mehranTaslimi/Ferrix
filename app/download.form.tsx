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
      // downloadUrl: "https://releases.ubuntu.com/24.04.2/ubuntu-24.04.2-desktop-amd64.iso"
      downloadUrl: "https://wallpaperswide.com/download/most_beautiful_mountain_scenery-wallpaper-3840x2160.jpg"
    },
  });

  useEffect(() => {
    listen("download_speed", (ev) => {
      console.log(ev.payload);
    });
    listen("downloaded_bytes", (ev) => {
      console.log(ev.payload);
    });
    listen("download_list", (ev) => {
      console.log(ev.payload);
    });
  }, [])

  useEffect(() => {
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
