import { Status } from "./types";
import React, { useState } from "react";
import { Button } from "./ui/button";
import { Pause, Play, X, Folder } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { revealItemInDir } from '@tauri-apps/plugin-opener'
import { RemoveDownloadDialog } from "./confirm-modal";

const buttonClassName = "h-9 w-9 font-medium transition-all duration-200";

interface ActionButtonsProps {
  status: Status;
  downloadId: number;
  filePath: string
  filename: string
  fileExist: boolean
}

export default function ActionButtons({
  status,
  downloadId,
  filePath,
  fileExist,
  filename
}: ActionButtonsProps) {
  const [confirm, setConfirm] = useState(false);
  const isResumeDisabled = status === Status.Writing;

  const handleToggleDownload = async () => {
    if (status === Status.Paused) {
      await invoke("resume_download", { id: downloadId });
    } else if (status === Status.Downloading) {
      await invoke("pause_download", { id: downloadId });
    }
  };

  const removeDownload = async (removeFile?: boolean) => {
    await invoke('remove_download', { id: downloadId, removeFile: !!removeFile })
  }

  const handleOpenFile = () => {
    revealItemInDir(filePath)
  };

  return (
    <>
      <RemoveDownloadDialog open={confirm} onOpenChange={() => setConfirm(prev => !prev)} onConfirm={(removeFile) => removeDownload(removeFile)} filename={filename} fileExist={fileExist} />
      <div className="flex gap-2">
        {status !== Status.Completed && status !== Status.Failed && (
          <Button
            onClick={handleToggleDownload}
            variant="outline"
            size="sm"
            className={buttonClassName}
            disabled={isResumeDisabled}
          >
            {status === Status.Paused ? (
              <div className="flex gap-2">
                <Play className="w-4 h-4" />
              </div>
            ) : (
              <Pause className="w-4 h-4" />
            )}
          </Button>
        )}
        {status !== Status.Downloading && status !== Status.Writing && (
          <Button
            onClick={() => setConfirm(true)}
            variant="outline"
            size="sm"
            className={buttonClassName}
          >
            <X className="w-4 h-4" />
          </Button>
        )}

        {
          status === Status.Completed && (
            <Button
              onClick={handleOpenFile}
              variant="outline"
              size="sm"
              className={buttonClassName}
            >
              <Folder className="w-4 h-4" />
            </Button>
          )
        }
      </div>
    </>
  );
}
