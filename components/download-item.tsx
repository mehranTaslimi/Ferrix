"use client";

import { memo } from "react";
import { } from "lucide-react";
import { Card, CardContent } from "@/components/ui/card";
import StatusAndSpeed from "./status-and-speed";
import { type DownloadType, Status } from "./types";
import ActionButtons from "./action-buttons";
import { FileIcon } from "./file-icon";

function DownloadItem({ download }: { download: DownloadType }) {
  const isDownloadCompleted = download.status === Status.Completed;

  return (
    <Card className="group hover:shadow-lg transition-all duration-200 border-0 shadow-sm bg-gradient-to-br from-card to-card/50">
      <CardContent className="px-6">
        <div className="space-y-2">
          <div className="flex items-start justify-between gap-3">
            <div className="flex items-center gap-3 min-w-0 flex-1">
              <div className="flex-shrink-0 p-2 rounded-lg bg-muted/50">
                <FileIcon extension={download.extension} />
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
            <ActionButtons fileExist={download.file_exist} filePath={download.file_path} downloadId={download.id} status={download.status} filename={download.file_name} />
          </div>

          {!isDownloadCompleted && (
            <StatusAndSpeed
              id={download.id}
              totalBytes={download.total_bytes}
              downloadedBytes={download.downloaded_bytes}
              status={download.status}
            />
          )}
        </div>
      </CardContent>
    </Card>
  );
}

export default memo(DownloadItem);
