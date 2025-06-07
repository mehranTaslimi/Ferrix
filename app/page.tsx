"use client";

import Image from "next/image";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Button } from "@/components/ui/button";

interface Progress {
  chunk: number;
  progress: number;
}

export default function Home() {
  const url =
    "https://dl3.soft98.ir/win/Windows.11.v24H2.Build.26100.4061.x64-VL.part1.rar?1749221346";
  const [progress, setProgress] = useState<Record<number, number>>({});

  useEffect(() => {
    (async () => {
      try {
        listen<Progress>("download_progress", (event) => {
          setProgress((prev) => ({
            ...prev,
            [event.payload.chunk]: event.payload.progress,
          }));
        });

        await invoke("download_file", {
          url,
        });
      } catch (e) {
        console.error(e);
      }
    })();

    return () => {
      invoke("cancel_download", { url });
    };
  }, []);

  // const handleCancelDownload = () => {
  //   invoke("cancel_download", { url });
  // };

  return (
    <div className="p-3">
      {/* <Button>Cancel Download</Button> */}
      {Object.entries(progress).map(([key, value]) => {
        return (
          <div className="mb-6" key={key}>
            <p className="font-light text-sm">chunk: {key}</p>
            <p>progress: {value}%</p>
          </div>
        );
      })}
    </div>
  );
}
