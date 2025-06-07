"use client";

import Image from "next/image";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import DownloadForm from "@/app/download.form";
import { Progress } from "@/components/ui/progress";
import DownloadProgress from "./download.progress";

interface Progress {
  chunk: number;
  progress: number;
}

export default function Home() {
  // const url =
  //   "https://dl3.soft98.ir/win/Windows.11.v24H2.Build.26100.4061.x64-VL.part1.rar?1749221346";
  const [downloadUrl, setDownloadUrl] = useState("");
  const [progress, setProgress] = useState<Record<number, number>>({});

  useEffect(() => {
    (async () => {
      try {
        if (downloadUrl) {
          listen<Progress>("download_progress", (event) => {
            setProgress((prev) => ({
              ...prev,
              [event.payload.chunk]: event.payload.progress,
            }));
          });

          await invoke("download_file", {
            url: downloadUrl,
          });
        } else {
          await invoke("cancel_download", { url: downloadUrl });
        }
      } catch (e) {
        console.error(e);
      }
    })();
  }, [downloadUrl]);

  // const handleCancelDownload = () => {
  //   invoke("cancel_download", { url });
  // };

  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-24">
      <div className="w-1/2">
        {downloadUrl ? (
          <DownloadProgress progress={progress} />
        ) : (
          <DownloadForm onSubmit={setDownloadUrl} />
        )}
      </div>
    </main>
  );
}
