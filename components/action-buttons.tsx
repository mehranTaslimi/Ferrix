"use client";

import React, { useCallback, useState } from "react";
import { Button } from "./ui/button";
import { Pause, Play, X, Folder } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { revealItemInDir } from "@tauri-apps/plugin-opener";
import { RemoveDownloadDialog } from "./confirm-modal";
import { useDownloads } from "./download-context";
import { Status } from "./types";

const buttonClassName = "h-9 w-9 font-medium transition-all duration-200";

interface ActionButtonsProps {
  status: Status;
  downloadId: number;
  filePath: string;
  filename: string;
  fileExist: boolean;
}

export default function ActionButtons({
  status,
  downloadId,
  filePath,
  fileExist,
  filename,
}: ActionButtonsProps) {
  const [confirmOpen, setConfirmOpen] = useState(false);
  const { removeDownload } = useDownloads();

  const isResumeDisabled = status === Status.Writing;
  const canToggle = status !== Status.Completed && status !== Status.Failed;
  const canRemove = status !== Status.Downloading && status !== Status.Writing;
  const canReveal = status === Status.Completed && fileExist;

  const handleToggleDownload = useCallback(async () => {
    if (status === Status.Paused) {
      await invoke("resume_download", { id: downloadId });
    } else if (status === Status.Downloading) {
      await invoke("pause_download", { id: downloadId });
    }
  }, [status, downloadId]);

  const handleRemoveConfirmed = useCallback(
    async (removeFile?: boolean) => {
      await removeDownload(downloadId, !!removeFile);
      setConfirmOpen(false);
    },
    [removeDownload, downloadId]
  );

  const handleOpenFile = useCallback(async () => {
    try {
      await revealItemInDir(filePath);
    } catch (e) {
      console.error("Failed to reveal in folder:", e);
    }
  }, [filePath]);

  return (
    <>
      <RemoveDownloadDialog
        open={confirmOpen}
        onOpenChange={setConfirmOpen}
        onConfirm={handleRemoveConfirmed}
        filename={filename}
        fileExist={fileExist}
      />

      <div className="flex gap-2">
        {canToggle && (
          <Button
            onClick={handleToggleDownload}
            variant="outline"
            size="sm"
            className={buttonClassName}
            disabled={isResumeDisabled}
            aria-label={status === Status.Paused ? "Resume" : "Pause"}
            title={status === Status.Paused ? "Resume" : "Pause"}
          >
            {status === Status.Paused ? (
              <Play className="w-4 h-4" />
            ) : (
              <Pause className="w-4 h-4" />
            )}
          </Button>
        )}

        {canRemove && (
          <Button
            onClick={() => setConfirmOpen(true)}
            variant="outline"
            size="sm"
            className={buttonClassName}
            aria-label="Remove"
            title="Remove"
          >
            <X className="w-4 h-4" />
          </Button>
        )}

        {canReveal && (
          <Button
            onClick={handleOpenFile}
            variant="outline"
            size="sm"
            className={buttonClassName}
            aria-label="Show in folder"
            title="Show in folder"
          >
            <Folder className="w-4 h-4" />
          </Button>
        )}
      </div>
    </>
  );
}
