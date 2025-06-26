"use client";

import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import Item from "./Item";
import { listen } from "@tauri-apps/api/event";
import Form from "./Form";
import { Download } from "./types";

export default function Page() {
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
