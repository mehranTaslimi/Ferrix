import { Status } from "./types";
import React, { ReactNode } from "react";
import { Button } from "./ui/button";
import { Delete, Pause, Play, X } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";

const buttonClassName =
  "h-9 w-9 font-medium transition-all duration-200 hover:scale-[1.02] bg-transparent";

interface ActionButtonsProps {
  status: Status;
  downloadId: number;
}

export default function ActionButtons({
  status,
  downloadId,
}: ActionButtonsProps) {
  const isResumeDisabled = status === Status.Writing;

  const handleToggleDownload = async () => {
    if (status === Status.Paused) {
      await invoke("resume_download", { id: downloadId });
    } else if (status === Status.Downloading) {
      await invoke("pause_download", { id: downloadId });
    }
  };

  const handleDelete = async () => {
    console.log("delete");
  };

  return (
    <div className="flex gap-2">
      {status !== Status.Completed && status !== Status.Failed && (
        <Button
          onClick={handleToggleDownload}
          variant="outline"
          size="sm"
          className={buttonClassName}
          disabled={isResumeDisabled}
        >
          {status === Status.Paused || status === Status.Queued ? (
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
          onClick={handleDelete}
          variant="outline"
          size="sm"
          className={buttonClassName}
        >
          <X className="w-4 h-4" />
        </Button>
      )}
    </div>
  );
}
