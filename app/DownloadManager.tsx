"use client";

import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { DownloadDashboard } from "@/components/download-dashboard";

// Download item type definition
interface DownloadItem {
  id: number;
  file_name: string;
  file_path: string;
  url: string;
  status: "queued" | "downloading" | "completed" | "failed" | "paused";
  total_bytes: number;
  downloaded_bytes: number;
  extension: string;
  content_type: string;
  created_at: string;
  chunk_count?: number;
  speed?: number;
}

export default function DownloadManager() {
  const [downloadList, setDownloadList] = useState<DownloadItem[]>([]);
  const unlistenRefs = useRef<UnlistenFn[]>([]);

  useEffect(() => {
    // Listen for download list updates
    const unlistenList = listen("download_list", (ev) => {
      console.log("okok", ev.payload);
      const payload = ev.payload as DownloadItem[];
      setDownloadList(payload);
    });

    // Cleanup listeners on component unmount
    return () => {
      unlistenList.then((unlisten) => unlisten());
      unlistenRefs.current.forEach((un) => un());
      unlistenRefs.current = [];
    };
  }, []);

  // Fetch download list on component mount
  useEffect(() => {
    (async () => {
      try {
        await invoke("get_download_list");
      } catch (e) {
        console.log(e);
      }
    })();
  }, []);

  // Listen for per download progress and speed updates
  useEffect(() => {
    // clear previous listeners
    unlistenRefs.current.forEach((un) => un());
    unlistenRefs.current = [];

    downloadList.forEach((item) => {
      listen<number>(`downloaded_bytes_${item.id}`, (ev) => {
        const bytes = ev.payload as number;
        setDownloadList((prev) =>
          prev.map((d) => (d.id === item.id ? { ...d, downloaded_bytes: bytes } : d))
        );
      }).then((un) => unlistenRefs.current.push(un));

      listen<{ speed: number; remaining_time: number }>(
        `speed_and_remaining_${item.id}`,
        (ev) => {
          const payload = ev.payload as { speed: number; remaining_time: number };
          setDownloadList((prev) =>
            prev.map((d) => (d.id === item.id ? { ...d, speed: payload.speed } : d))
          );
        }
      ).then((un) => unlistenRefs.current.push(un));
    });
  }, [downloadList.map((d) => d.id).join(",")]);

  // Add download with URL and chunk count
  async function addDownload(url: string, chunkCount: number) {
    try {
      await invoke("add_download_queue", {
        url,
        chunkCount,
      });
    } catch (e) {
      console.log(e);
    }
  }

  return (
    <DownloadDashboard data={downloadList} onAddDownload={addDownload} />
  );
}
