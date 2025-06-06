"use client";

import Image from "next/image";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface Progress {
  chunk: number;
  progress: number;
}

export default function Home() {
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
          url: "https://dl3.soft98.ir/win/Windows.11.v24H2.Build.26100.4061.x64-VL.part1.rar?1749221346",
        });
      } catch (e) {
        console.error(e);
      }
    })();
  }, []);

  return (
    <div className="p-3">
      {Object.entries(progress).map(([key, value]) => {
        return (
          <div className="mb-6">
            <p className="font-light text-sm">chunk: {key}</p>
            <p>progress: {value}%</p>
          </div>
        );
      })}
    </div>
  );
}
