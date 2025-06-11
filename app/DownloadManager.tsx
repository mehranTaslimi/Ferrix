"use client"

import { useState, useEffect } from "react"
import { invoke } from "@tauri-apps/api/core"
import { listen } from "@tauri-apps/api/event"
import { DownloadDataTable } from "@/components/data-table"

// Download item type definition
interface DownloadItem {
  id: number
  file_name: string
  file_path: string
  url: string
  status: "queued" | "downloading" | "completed" | "failed"
  total_bytes: number
  downloaded_bytes: number
  extension: string
  content_type: string
  created_at: string
  chunk_count?: number
  speed?: number
}

export default function DownloadManager() {
  const [downloadList, setDownloadList] = useState<DownloadItem[]>([])
  const [downloadSpeeds, setDownloadSpeeds] = useState<Record<number, number>>({})
  const [downloadedBytes, setDownloadedBytes] = useState<Record<number, number>>({})

  useEffect(() => {
    // Listen for download speed updates
    const unlistenSpeed = listen("download_speed", (ev) => {
      const payload = ev.payload as Record<number, number>
      setDownloadSpeeds((prev) => ({ ...prev, ...payload }))
    })

    // Listen for downloaded bytes updates
    const unlistenBytes = listen("downloaded_bytes", (ev) => {
      const payload = ev.payload as Record<number, number>
      setDownloadedBytes((prev) => ({ ...prev, ...payload }))

      // Update the download list with the new downloaded bytes
      setDownloadList((prev) =>
        prev.map((item) => {
          if (payload[item.id] !== undefined) {
            return { ...item, downloaded_bytes: payload[item.id] }
          }
          return item
        }),
      )
    })

    // Listen for download list updates
    const unlistenList = listen("download_list", (ev) => {
      const payload = ev.payload as DownloadItem[]
      setDownloadList(payload)
    })

    // Cleanup listeners on component unmount
    return () => {
      unlistenSpeed.then((unlisten) => unlisten())
      unlistenBytes.then((unlisten) => unlisten())
      unlistenList.then((unlisten) => unlisten())
    }
  }, [])

  // Fetch download list on component mount
  useEffect(() => {
    ; (async () => {
      try {
        await invoke("get_download_list")
      } catch (e) {
        console.log(e)
      }
    })()
  }, [])

  // Add download with URL and chunk count
  async function addDownload(url: string, chunkCount: number) {
    try {
      await invoke("add_download_queue", {
        url,
        chunkCount,
      })
    } catch (e) {
      console.log(e)
    }
  }

  // Combine download list with speeds
  const downloadsWithSpeeds = downloadList.map((item) => ({
    ...item,
    speed: downloadSpeeds[item.id] || 0,
  }))

  return (
    <div className="p-4 max-w-7xl mx-auto">
      <DownloadDataTable data={downloadsWithSpeeds} onAddDownload={addDownload} />
    </div>
  )
}
