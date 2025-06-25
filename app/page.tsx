"use client";

import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import Item from "./Item";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { listen } from "@tauri-apps/api/event";
import Form from "./Form";

export interface Download {
  id: number;
  url: string;
  total_bytes: number;
  status: Status;
  created_at: Date;
  downloaded_bytes: number;
  chunk_count: number;
  file_path: string;
  file_name: string;
  content_type: ContentType;
  extension: Extension;
}

export enum ContentType {
  ApplicationXRarCompressed = "application/x-rar-compressed",
  ImageJPEG = "image/jpeg",
  VideoMp4 = "video/mp4",
}

export enum Extension {
  Jpg = "jpg",
  Mp4 = "mp4",
  Rar = "rar",
}

export enum Status {
  Downloading = "downloading",
  Completed = "completed",
  Queued = "queued",
  Paused = "paused",
  Failed = "failed"
}


export default function Home() {
  const [downloadList, setDownloadList] = useState<Download[]>([]);

  useEffect(() => {

    const unlisten = listen<Download>("download_item", (ev) => {
      setDownloadList(prev => {
        const clone = structuredClone(prev);
        const index = clone.map(i => i.id).indexOf(ev.payload.id);

        if (index > -1) {
          clone[index] = ev.payload;
        } else {
          clone.unshift(ev.payload)
        }

        return clone;
      })
    });

    (async () => {
      const downloadList = await invoke<Download[]>("get_download_list");
      setDownloadList(downloadList);
    })();


    return () => {
      unlisten.then(fn => fn())
    }
  }, [])

  return (
    <div className="container m-auto pt-10">
      <Form />
      <ul className="flex flex-wrap gap-3">
        {
          downloadList.map(item => {
            return (
              <Item download={item} key={item.id} />
            )
          })
        }
      </ul>
    </div>
  );
}
