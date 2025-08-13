"use client";

import { memo } from "react";
import { Card, CardContent } from "@/components/ui/card";
import { type DownloadType, Status } from "./types";
import ActionButtons from "./action-buttons";
import { FileIcon } from "./file-icon";
import StatusAndSpeed from "./status-and-speed";

function DownloadItem({ download }: { download: DownloadType }) {
  const isDone = download.status === Status.Completed;

  return (
    <Card className="group border border-border/50 bg-gradient-to-br from-card to-card/60 shadow-sm hover:shadow-md transition">
      <CardContent className="px-5">
        <div className="flex items-start justify-between gap-3">
          <div className="flex items-center gap-3 min-w-0 flex-1">
            <div className="flex-shrink-0 p-2 rounded-lg bg-muted/50">
              <FileIcon extension={download.extension} />
            </div>
            <div className="min-w-0">
              <h3 className="font-medium text-sm leading-tight truncate" title={download.file_name}>
                {download.file_name}
              </h3>
              <p className="text-[11px] text-muted-foreground">{(download.total_bytes / (1024 * 1024)).toFixed(1)} MB</p>
            </div>
          </div>

          <ActionButtons
            fileExist={download.file_exist}
            filePath={download.file_path}
            downloadId={download.id}
            status={download.status}
            filename={download.file_name}
          />
        </div>

        {!isDone && (
          <div className="mt-3">
            <StatusAndSpeed
              errorMessage={download.error_message}
              id={download.id}
              totalBytes={download.total_bytes}
              downloadedBytes={download.downloaded_bytes}
              status={download.status}
            />
          </div>
        )}
      </CardContent>
    </Card>
  );
}

export default memo(DownloadItem);
