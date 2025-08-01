"use client";

import { memo, useEffect, useMemo, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { Progress } from "@/components/ui/progress";
import { Clock, Download, HardDrive } from "lucide-react";
import { Status } from "./types";
import StatusIndicator from "./status-indicator";
import clsx from "clsx";

function getStatusColor(status: Status) {
  switch (status) {
    case Status.Downloading:
      return "bg-blue-500";
    case Status.Completed:
      return "bg-green-500";
    case Status.Failed:
      return "bg-red-500";
    case Status.Error:
      return "bg-red-500";
    case Status.Paused:
      return "bg-yellow-500";
    case Status.Queued:
      return "bg-gray-500";
    case Status.Writing:
      return "bg-purple-500";
    default:
      return "bg-gray-500";
  }
}
interface SpeedAndRemaining {
  speed: number;
  remaining_time: number;
  diskSpeed: number;
}

function StatusAndSpeed({
  id,
  totalBytes,
  status,
  downloadedBytes: initialDownloadedBytes,
}: {
  id: number;
  totalBytes: number;
  status: Status;
  downloadedBytes: number;
}) {
  const [downloadedBytes, setDownloadedBytes] = useState(
    initialDownloadedBytes
  );
  const [wroteBytes, setWroteBytes] = useState(initialDownloadedBytes);
  const [speedAndRemaining, setSpeedAndRemaining] = useState<SpeedAndRemaining>(
    {
      speed: 0,
      diskSpeed: 0,
      remaining_time: 0,
    }
  );

  useEffect(() => {
    const unListen1 = listen<number>(`downloaded_bytes_${id}`, (ev) => {
      setDownloadedBytes(ev.payload);
    });

    const unListen2 = listen<SpeedAndRemaining>(
      `speed_and_remaining_${id}`,
      (ev) => {
        setSpeedAndRemaining((prev) => ({
          ...prev,
          ...ev.payload,
        }));
      }
    );

    const unListen3 = listen<number>(`disk_speed_${id}`, (ev) => {
      setSpeedAndRemaining((prev) => ({ ...prev, diskSpeed: ev.payload }));
    });

    const unListen4 = listen<number>(`wrote_bytes_${id}`, (ev) => {
      setWroteBytes(ev.payload);
    });

    return () => {
      unListen1.then((fn) => fn());
      unListen2.then((fn) => fn());
      unListen3.then((fn) => fn());
      unListen4.then((fn) => fn());
    };
  }, [id]);

  useEffect(() => {
    setDownloadedBytes(initialDownloadedBytes);
    setWroteBytes(initialDownloadedBytes);
  }, [initialDownloadedBytes]);

  const downloadProgress = useMemo(() => {
    return Math.round((downloadedBytes / totalBytes) * 100);
  }, [downloadedBytes, totalBytes]);

  const writeProgress = useMemo(() => {
    return Math.round((wroteBytes / totalBytes) * 100);
  }, [wroteBytes, totalBytes]);

  const remainingTime = useMemo(() => {
    const second = Math.round(speedAndRemaining.remaining_time);
    const minute = Math.round(speedAndRemaining.remaining_time / 60);
    const hour = Math.round(speedAndRemaining.remaining_time / 60 / 60);
    const day = Math.round(speedAndRemaining.remaining_time / 60 / 60 / 24);

    if (day >= 1) {
      return day > 1 ? day + " days" : day + " day";
    }
    if (hour >= 1) {
      return hour > 1 ? hour + " hours" : hour + " hour";
    }
    if (minute >= 1) {
      return minute > 1 ? minute + " minutes" : minute + " minute";
    }

    return second > 1 ? second + " seconds" : second + " second";
  }, [speedAndRemaining.remaining_time]);

  const speed = useMemo(() => {
    const kb = speedAndRemaining.speed;
    const mb = speedAndRemaining.speed / 1000;
    const gb = speedAndRemaining.speed / 1000 / 1000;

    if (gb >= 1) {
      return gb.toFixed(1) + " GB/s";
    }
    if (mb >= 1) {
      return mb.toFixed(1) + " MB/s";
    }

    return kb.toFixed(1) + " KB/s";
  }, [speedAndRemaining.speed]);

  const diskSpeed = useMemo(() => {
    const kb = speedAndRemaining.diskSpeed;
    const mb = speedAndRemaining.diskSpeed / 1000;
    const gb = speedAndRemaining.diskSpeed / 1000 / 1000;

    if (gb >= 1) {
      return gb.toFixed(1) + " GB/s";
    }
    if (mb >= 1) {
      return mb.toFixed(1) + " MB/s";
    }

    return kb.toFixed(1) + " KB/s";
  }, [speedAndRemaining.diskSpeed]);

  const isWriting = status === Status.Writing;
  const isDownloading = status === Status.Downloading;

  return (
    <div className="space-y-3">
      <div className="space-y-2">
        {isWriting ? (
          <>
            <div className="flex justify-between items-center">
              <div className="flex gap-2">
                <span className="text-sm font-medium">
                  Writing: {writeProgress}%
                </span>
                <StatusIndicator status={status} />
              </div>

              <span className="text-xs text-muted-foreground">
                {(wroteBytes / (1024 * 1024)).toFixed(1)} MB
              </span>
            </div>

            <Progress
              value={writeProgress}
              className="h-2 bg-muted/30 [&>div]:bg-purple-500"
            />
          </>
        ) : (
          <>
            <div className="flex justify-between items-center">
              <div className="flex gap-2">
                <span className="text-sm font-medium">{downloadProgress}%</span>
                <StatusIndicator status={status} />
              </div>
              <span className="text-xs text-muted-foreground">
                {(downloadedBytes / (1024 * 1024)).toFixed(1)} MB
              </span>
            </div>

            <Progress value={downloadProgress} className={clsx("h-2 bg-muted/30", {
              "[&>div]:bg-red-600/50": status === Status.Error || status === Status.Failed
            })} />
          </>
        )}
      </div>

      {isDownloading && (
        <div className="flex items-center gap-3 text-xs">
          <div className="flex items-center gap-1.5 px-2 py-1 rounded-md bg-muted/50">
            <Download className="w-3 h-3 text-blue-500" />
            <span className="font-medium">{speed}</span>
          </div>
          <div className="flex items-center gap-1.5 px-2 py-1 rounded-md bg-muted/50">
            <HardDrive className="w-3 h-3 text-green-500" />
            <span className="font-medium">{diskSpeed}</span>
          </div>
          <div className="flex items-center gap-1.5 px-2 py-1 rounded-md bg-muted/50">
            <Clock className="w-3 h-3 text-orange-500" />
            <span className="font-medium">{remainingTime}</span>
          </div>
        </div>
      )}

      {isWriting && (
        <div className="flex items-center gap-3 text-xs">
          <div className="flex items-center gap-1.5 px-2 py-1 rounded-md bg-muted/50">
            <HardDrive className="w-3 h-3 text-purple-500" />
            <span className="font-medium">{diskSpeed}</span>
          </div>
          <div className="flex items-center gap-1.5 px-2 py-1 rounded-md bg-purple-100 dark:bg-purple-900/30">
            <span className="text-purple-600 dark:text-purple-400 font-medium">
              Writing to disk...
            </span>
          </div>
        </div>
      )}
    </div>
  );
}

export default memo(StatusAndSpeed);
