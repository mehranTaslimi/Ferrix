"use client";
import {
  Download,
  Pause,
  Play,
  Trash2,
  FileText,
  ImageIcon,
  Video,
  ChevronDown,
  Calendar,
  Link,
  Package,
} from "lucide-react";
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Progress } from "@/components/ui/progress";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type Action = "delete" | "pause" | "resume";
export interface DownloadItem {
  id: number;
  file_name: string;
  file_path: string;
  url: string;
  status: "queued" | "downloading" | "completed" | "failed";
  total_bytes: number;
  downloaded_bytes: number;
  extension: string;
  content_type: string;
  created_at: string;
  chunk_count?: number;
  speed?: number;
}

interface DownloadCardProps {
  item: DownloadItem;
}

const getFileIcon = (contentType: string, extension: string) => {
  if (
    contentType.startsWith("video/") ||
    ["mp4", "avi", "mkv", "mov", "wmv"].includes(extension.toLowerCase())
  ) {
    return Video;
  }
  if (
    contentType.startsWith("image/") ||
    ["jpg", "jpeg", "png", "gif", "webp", "svg"].includes(
      extension.toLowerCase()
    )
  ) {
    return ImageIcon;
  }
  if (
    contentType.includes("pdf") ||
    ["pdf", "doc", "docx", "txt", "rtf"].includes(extension.toLowerCase())
  ) {
    return FileText;
  }
  return Download;
};

const getStatusIndicator = (status: string) => {
  switch (status) {
    case "queued":
      return "bg-gray-400/50";
    case "downloading":
      return "bg-blue-500/50";
    case "completed":
      return "bg-green-500/50";
    case "failed":
      return "bg-red-500/50";
    default:
      return "bg-gray-400/50";
  }
};

const formatBytes = (bytes: number): string => {
  if (bytes === 0) return "0 Bytes";
  const k = 1024;
  const sizes = ["Bytes", "KB", "MB", "GB", "TB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return (
    Number.parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i]
  );
};

const formatSpeed = (bytesPerSecond: number): string => {
  return formatBytes(bytesPerSecond) + "/s";
};

const calculateTimeRemaining = (
  totalBytes: number,
  downloadedBytes: number,
  speed: number
): string => {
  if (!speed || speed === 0) return "Unknown";
  const remainingBytes = totalBytes - downloadedBytes;
  const secondsRemaining = remainingBytes / speed;

  if (secondsRemaining < 60) {
    return `${Math.round(secondsRemaining)}s`;
  } else if (secondsRemaining < 3600) {
    return `${Math.round(secondsRemaining / 60)}m`;
  } else {
    const hours = Math.floor(secondsRemaining / 3600);
    const minutes = Math.round((secondsRemaining % 3600) / 60);
    return `${hours}h ${minutes}m`;
  }
};

export function DownloadCard({ item }: DownloadCardProps) {
  const [isExpanded, setIsExpanded] = useState(false);
  const FileIcon = getFileIcon(item.content_type, item.extension);
  const statusIndicator = getStatusIndicator(item.status);
  const progress =
    item.total_bytes > 0
      ? Math.round((item.downloaded_bytes / item.total_bytes) * 100)
      : 0;
  const formattedSize = formatBytes(item.total_bytes);
  const formattedDownloaded = formatBytes(item.downloaded_bytes);
  const formattedSpeed = item.speed ? formatSpeed(item.speed) : null;
  const timeRemaining =
    item.speed && item.status === "downloading"
      ? calculateTimeRemaining(
          item.total_bytes,
          item.downloaded_bytes,
          item.speed
        )
      : null;

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString("en-US", {
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  };

  async function handlePause(action: Action, id: number) {
    try {
      await invoke("pause_download", { id });
    } catch (e) {
      console.log(e);
    }
  }
  const handleDelete = (action: Action, id: number) => {};
  const handleResume = (action: Action, id: number) => {};
  console.log(item);
  return (
    <Card className="group relative overflow-hidden card-glass glass-morphism-hover transition-all duration-300">
      {/* Enhanced status indicator */}
      <div
        className={`absolute top-0 left-0 w-full h-1.5 ${statusIndicator}`}
      />

      <div className="relative">
        {/* Main card content */}
        <div
          className="p-4 cursor-pointer"
          onClick={() => setIsExpanded(!isExpanded)}
        >
          <div className="flex items-start justify-between">
            <div className="flex items-center gap-3 flex-1 min-w-0">
              <div className="flex h-10 w-10 items-center justify-center rounded-xl glass-morphism">
                <FileIcon className="h-5 w-5 text-gray-900 dark:text-foreground/80" />
              </div>
              <div className="flex-1 min-w-0">
                <h3
                  className="font-semibold text-sm truncate text-gray-950 dark:text-foreground"
                  title={item.file_name}
                >
                  {item.file_name}
                </h3>
                <p className="text-xs text-gray-800 dark:text-muted-foreground font-medium">
                  {formattedSize}
                </p>
              </div>
            </div>

            <Button
              variant="ghost"
              size="sm"
              className="h-8 w-8 p-0 glass-morphism-hover rounded-lg"
              onClick={(e) => {
                e.stopPropagation();
                setIsExpanded(!isExpanded);
              }}
            >
              <ChevronDown
                className={`h-4 w-4 text-gray-900 dark:text-foreground/80 transition-transform duration-200 ${
                  isExpanded ? "rotate-180" : ""
                }`}
              />
            </Button>
          </div>

          <div className="mt-4 space-y-3">
            <div className="flex items-center justify-between text-xs">
              <span className="capitalize text-gray-800 dark:text-muted-foreground font-medium">
                {item.status}
              </span>
              {item.status === "downloading" && formattedSpeed && (
                <span className="text-gray-800 dark:text-muted-foreground font-medium">
                  {formattedSpeed}
                </span>
              )}
            </div>

            <Progress value={progress} className="h-2 glass-progress" />

            <div className="flex items-center justify-between">
              <span className="text-xs text-gray-800 dark:text-muted-foreground font-medium">
                {progress}%
              </span>
              {timeRemaining && item.status === "downloading" && (
                <span className="text-xs text-gray-800 dark:text-muted-foreground font-medium">
                  {timeRemaining} remaining
                </span>
              )}
            </div>
          </div>
        </div>

        {/* Expanded content */}
        <div
          className={`overflow-hidden transition-all duration-300 ease-in-out ${
            isExpanded ? "max-h-96 opacity-100" : "max-h-0 opacity-0"
          }`}
        >
          <div className="px-4 pb-4 border-t border-gray-300 dark:border-white/15 glass-text-container">
            <div className="pt-4 space-y-4">
              {/* Download details */}
              <div className="grid grid-cols-2 gap-4 text-xs">
                <div className="space-y-1">
                  <div className="flex items-center gap-2 text-gray-800 dark:text-muted-foreground">
                    <Package className="h-3 w-3" />
                    <span className="font-medium">Downloaded</span>
                  </div>
                  <div className="font-semibold text-gray-950 dark:text-foreground">
                    {formattedDownloaded} / {formattedSize}
                  </div>
                </div>

                <div className="space-y-1">
                  <div className="flex items-center gap-2 text-gray-800 dark:text-muted-foreground">
                    <Calendar className="h-3 w-3" />
                    <span className="font-medium">Created</span>
                  </div>
                  <div className="font-semibold text-gray-950 dark:text-foreground">
                    {formatDate(item.created_at)}
                  </div>
                </div>
              </div>

              {/* File details */}
              <div className="space-y-2">
                <div className="flex items-center gap-2 text-xs text-gray-800 dark:text-muted-foreground">
                  <Link className="h-3 w-3" />
                  <span className="font-medium">Source URL</span>
                </div>
                <div
                  className="text-xs font-mono glass-morphism rounded-lg p-3 truncate text-gray-950 dark:text-foreground/90 font-medium"
                  title={item.url}
                >
                  {item.url}
                </div>
              </div>

              {/* File path */}
              <div className="space-y-2">
                <div className="text-xs text-gray-800 dark:text-muted-foreground font-medium">
                  File Path
                </div>
                <div
                  className="text-xs font-mono glass-morphism rounded-lg p-3 truncate text-gray-950 dark:text-foreground/90 font-medium"
                  title={item.file_path}
                >
                  {item.file_path}
                </div>
              </div>

              {/* Technical details */}
              <div className="grid grid-cols-2 gap-3 text-xs">
                <div>
                  <span className="text-gray-800 dark:text-muted-foreground font-medium">
                    Type:{" "}
                  </span>
                  <span className="font-semibold text-gray-950 dark:text-foreground">
                    {item.content_type}
                  </span>
                </div>
                {item.chunk_count && (
                  <div>
                    <span className="text-gray-800 dark:text-muted-foreground font-medium">
                      Chunks:{" "}
                    </span>
                    <span className="font-semibold text-gray-950 dark:text-foreground">
                      {item.chunk_count}
                    </span>
                  </div>
                )}
              </div>
            </div>
          </div>
        </div>

        {/* Action buttons */}
        <div className="px-4 pb-4">
          <div className="flex items-center gap-2">
            {item.status === "downloading" && (
              <Button
                size="sm"
                variant="ghost"
                onClick={() => handlePause("pause", item.id)}
                className="h-7 px-3 glass-morphism-hover rounded-lg text-gray-900 dark:text-foreground/90 font-medium"
              >
                <Pause className="h-3 w-3 mr-1" />
                Pause
              </Button>
            )}
            {(item.status === "queued" || item.status === "failed") && (
              <Button
                size="sm"
                variant="ghost"
                onClick={() => handleResume("resume", item.id)}
                className="h-7 px-3 glass-morphism-hover rounded-lg text-gray-900 dark:text-foreground/90 font-medium"
              >
                <Play className="h-3 w-3 mr-1" />
                {item.status === "failed" ? "Retry" : "Start"}
              </Button>
            )}
            {item.status === "completed" && (
              <Button
                size="sm"
                variant="ghost"
                onClick={() => {}}
                className="h-7 px-3 glass-morphism-hover rounded-lg text-gray-900 dark:text-foreground/90 font-medium"
              >
                <Download className="h-3 w-3 mr-1" />
                Open
              </Button>
            )}
            <Button
              size="sm"
              variant="ghost"
              onClick={() => handleDelete("delete", item.id)}
              className="h-7 px-3 glass-morphism-hover rounded-lg text-red-700 hover:text-red-800 hover:bg-red-500/15 border border-red-500/25 hover:border-red-500/40 transition-all duration-200 font-medium"
            >
              <Trash2 className="h-3 w-3 mr-1" />
              Delete
            </Button>
          </div>
        </div>
      </div>
    </Card>
  );
}
