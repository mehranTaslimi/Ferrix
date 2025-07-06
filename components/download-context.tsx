"use client"

import { createContext, useContext, useEffect, useState, type ReactNode } from "react"
import { invoke } from "@tauri-apps/api/core"
import { listen } from "@tauri-apps/api/event"
import type { DownloadType } from "./types"

interface DownloadContextType {
    downloads: DownloadType[]
    filteredDownloads: DownloadType[]
    selectedMimeType: string | null
    isLoading: boolean
    setSelectedMimeType: (mimeType: string | null) => void
}

const DownloadContext = createContext<DownloadContextType | undefined>(undefined)

export function useDownloads() {
    const context = useContext(DownloadContext)
    if (context === undefined) {
        throw new Error("useDownloads must be used within a DownloadProvider")
    }
    return context
}

export function DownloadProvider({ children }: { children: ReactNode }) {
    const [downloads, setDownloads] = useState<DownloadType[]>([])
    const [selectedMimeType, setSelectedMimeType] = useState<string | null>(null)
    const [isLoading, setIsLoading] = useState(true)

    useEffect(() => {
        listen("error", (ev) => {
            console.log(ev)
        })
    }, [])

    useEffect(() => {

        const unlisten = listen<DownloadType>("download_item", (ev) => {
            setDownloads((prev) => {
                const clone = structuredClone(prev)
                const index = clone.map((i) => i.id).indexOf(ev.payload.id)

                if (index > -1) {
                    clone[index] = ev.payload
                } else {
                    clone.unshift(ev.payload)
                }

                return clone
            })
        })
            ; (async () => {
                try {
                    const downloadList = await invoke<DownloadType[]>("get_download_list")
                    setDownloads(downloadList)
                } catch (error) {
                    console.error("Failed to load downloads:", error)
                } finally {
                    setIsLoading(false)
                }
            })()

        return () => {
            unlisten.then((fn) => fn())
        }
    }, [])

    // Filter downloads based on selected MIME type
    const filteredDownloads = selectedMimeType
        ? downloads.filter((download) => {
            // For grouped categories, check if the download's MIME type starts with the category
            if (selectedMimeType.startsWith("image/")) {
                return download.content_type.startsWith("image/")
            }
            if (selectedMimeType.startsWith("video/")) {
                return download.content_type.startsWith("video/")
            }
            if (selectedMimeType.startsWith("audio/")) {
                return download.content_type.startsWith("audio/")
            }

            return download.content_type === selectedMimeType
        })
        : downloads

    return (
        <DownloadContext.Provider
            value={{
                downloads,
                filteredDownloads,
                selectedMimeType,
                isLoading,
                setSelectedMimeType,
            }}
        >
            {children}
        </DownloadContext.Provider>
    )
}
