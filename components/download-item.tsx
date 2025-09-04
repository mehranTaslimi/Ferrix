'use client';

import { memo } from 'react';

import { Card, CardContent } from '@/components/ui/card';

import ActionButtons from './action-buttons';
import { FileIcon } from './file-icon';
import StatusAndSpeed from './status-and-speed';
import { type DownloadType, Status } from './types';

function DownloadItem({ download }: { download: DownloadType }) {
  const isDone = download.status === Status.Completed;

  return (
    <Card className="group border-border/50 from-card to-card/60 border bg-gradient-to-br shadow-sm transition hover:shadow-md">
      <CardContent className="px-5">
        <div className="flex items-start justify-between gap-3">
          <div className="flex min-w-0 flex-1 items-center gap-3">
            <div className="bg-muted/50 flex-shrink-0 rounded-lg p-2">
              <FileIcon extension={download.extension} />
            </div>
            <div className="min-w-0">
              <h3 className="truncate text-sm leading-tight font-medium" title={download.file_name}>
                {download.file_name}
              </h3>
              <p className="text-muted-foreground text-[11px]">
                {(download.total_bytes / (1024 * 1024)).toFixed(1)} MB
              </p>
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
