"use client";

import { memo } from "react";
import {
  Pause,
  Play,
  DownloadIcon,
  FileText,
  ImageIcon,
  Archive,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { invoke } from "@tauri-apps/api/core";
import StatusAndSpeed from "./status-and-speed";
import { type DownloadType, Status, Extension } from "./types";
import ActionButtons from "./action-buttons";

function getFileIcon(extension: Extension) {
  const iconClass = "w-5 h-5";
  switch (extension) {
    case Extension.Mp4:
      return <DownloadIcon className={iconClass} />;
    case Extension.Jpg:
      return <ImageIcon className={iconClass} />;
    case Extension.Rar:
      return <Archive className={iconClass} />;
    default:
      return <FileText className={iconClass} />;
  }
}

function DownloadItem({ download }: { download: DownloadType }) {
  return (
    <Card className="group hover:shadow-lg transition-all duration-200 border-0 shadow-sm bg-gradient-to-br from-card to-card/50">
      <CardContent className="px-6">
        <div className="space-y-2">
          {/* Header */}
          <div className="flex items-start justify-between gap-3">
            <div className="flex items-center gap-3 min-w-0 flex-1">
              <div className="flex-shrink-0 p-2 rounded-lg bg-muted/50">
                {getFileIcon(download.extension)}
              </div>
              <div className="min-w-0 flex-1">
                <h3
                  className="font-semibold text-sm leading-tight truncate mb-1"
                  title={download.file_name}
                >
                  {download.file_name}
                </h3>
                <p className="text-xs text-muted-foreground">
                  {(download.total_bytes / (1024 * 1024)).toFixed(1)} MB
                </p>
              </div>
            </div>
            <ActionButtons downloadId={download.id} status={download.status} />
          </div>

          {/* Progress and Speed Section - Handled by StatusAndSpeed component */}
          <StatusAndSpeed
            id={download.id}
            totalBytes={download.total_bytes}
            downloadedBytes={download.downloaded_bytes}
            status={download.status}
          />
        </div>
      </CardContent>
    </Card>
  );
}

export default memo(DownloadItem);
