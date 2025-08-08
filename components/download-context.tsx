"use client";

import {
    createContext,
    useContext,
    useEffect,
    useMemo,
    useState,
    type ReactNode,
} from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { DownloadType } from "./types";
import { labelForMime } from "@/utils/mime-utils";

interface DownloadContextType {
    downloads: DownloadType[];
    filteredDownloads: DownloadType[];
    selectedMimeType: string | null;
    isLoading: boolean;
    setSelectedMimeType: (mimeType: string | null) => void;
    removeDownload: (id: number, removeFile?: boolean) => Promise<void>;
}

const DownloadContext = createContext<DownloadContextType | undefined>(undefined);

export function useDownloads() {
    const ctx = useContext(DownloadContext);
    if (!ctx) throw new Error("useDownloads must be used within a DownloadProvider");
    return ctx;
}

export function DownloadProvider({ children }: { children: ReactNode }) {
    const [downloads, setDownloads] = useState<DownloadType[]>([]);
    const [selectedMimeType, setSelectedMimeType] = useState<string | null>(null);
    const [isLoading, setIsLoading] = useState(true);

    const norm = (mime?: string) =>
        (mime || "application/octet-stream").toLowerCase();

    useEffect(() => {
        const stopError = listen("error", (ev) => {
            console.log(ev);
        });

        const stopItem = listen<DownloadType>("download_item", (ev) => {
            setDownloads((prev) => {
                const clone = structuredClone(prev);
                const idx = clone.findIndex((i) => i.id === ev.payload.id);
                if (idx > -1) clone[idx] = ev.payload;
                else clone.unshift(ev.payload);
                return clone;
            });
        });

        (async () => {
            try {
                const list = await invoke<DownloadType[]>("get_download_list");
                setDownloads(list);
            } catch (err) {
                console.error("Failed to load downloads:", err);
            } finally {
                setIsLoading(false);
            }
        })();

        return () => {
            stopError.then((fn) => fn());
            stopItem.then((fn) => fn());
        };
    }, []);

    const removeDownload = async (id: number, removeFile = false) => {
        try {
            await invoke("remove_download", { id, removeFile });
            // Optimistic UI update
            setDownloads((prev) => prev.filter((d) => d.id !== id));
        } catch (err) {
            console.error("Failed to remove download:", err);
        }
    };

    const filteredDownloads = useMemo(() => {
        if (!selectedMimeType) return downloads;
        const selectedCategory = labelForMime(selectedMimeType);
        return downloads.filter(
            (d) => labelForMime(norm(d.content_type)) === selectedCategory
        );
    }, [downloads, selectedMimeType]);

    return (
        <DownloadContext.Provider
            value={{
                downloads,
                filteredDownloads,
                selectedMimeType,
                isLoading,
                setSelectedMimeType,
                removeDownload,
            }}
        >
            {children}
        </DownloadContext.Provider>
    );
}
