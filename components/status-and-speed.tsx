"use client";

import { memo, useEffect, useMemo, useRef, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { Progress } from "@/components/ui/progress";
import { Clock, Download, HardDrive } from "lucide-react";
import { Status } from "./types";
import clsx from "clsx";
import { SpeedChart, type SpeedPoint } from "./speed-chart";

interface SpeedAndRemaining {
  speed: number;
  remaining_time: number;
  diskSpeed: number;
}

const MAX_POINTS = 90;

function fmtThroughputKB(kb: number) {
  const mb = kb / 1000;
  const gb = mb / 1000;
  if (gb >= 1) return `${gb.toFixed(1)} GB/s`;
  if (mb >= 1) return `${mb.toFixed(1)} MB/s`;
  return `${kb.toFixed(0)} KB/s`;
}

function fmtRemaining(s: number) {
  const sec = Math.round(s);
  const min = Math.round(s / 60);
  const hr = Math.round(s / 3600);
  const day = Math.round(s / 86400);
  if (day >= 1) return day > 1 ? `${day} days` : "1 day";
  if (hr >= 1) return hr > 1 ? `${hr} hours` : "1 hour";
  if (min >= 1) return min > 1 ? `${min} minutes` : "1 minute";
  return sec > 1 ? `${sec} seconds` : "1 second";
}

function StatusPill({ status }: { status: Status }) {
  const map: Record<Status, { label: string; cls: string }> = {
    [Status.Downloading]: { label: "Downloading", cls: "bg-blue-500/10 text-blue-500 border-blue-500/20" },
    [Status.Writing]: { label: "Writing", cls: "bg-purple-500/10 text-purple-500 border-purple-500/20" },
    [Status.Paused]: { label: "Paused", cls: "bg-yellow-500/10 text-yellow-600 border-yellow-500/20" },
    [Status.Queued]: { label: "Queued", cls: "bg-muted text-foreground/70 border-transparent" },
    [Status.Completed]: { label: "Completed", cls: "bg-emerald-500/10 text-emerald-500 border-emerald-500/20" },
    [Status.Failed]: { label: "Failed", cls: "bg-red-500/10 text-red-500 border-red-500/20" },
    [Status.Error]: { label: "Error", cls: "bg-red-500/10 text-red-500 border-red-500/20" },
    [Status.Trying]: { label: "Trying", cls: "bg-amber-500/10 text-amber-500 border-amber-500/20" },
  };

  const { label, cls } = map[status];

  return (
    <span className={clsx("inline-flex items-center rounded-full border px-2 py-0.5 text-[10px] font-medium", cls)}>
      {label}
    </span>
  );
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
  const [downloadedBytes, setDownloadedBytes] = useState(initialDownloadedBytes);
  const [wroteBytes, setWroteBytes] = useState(initialDownloadedBytes);
  const [sr, setSr] = useState<SpeedAndRemaining>({ speed: 0, diskSpeed: 0, remaining_time: 0 });


  const [series, setSeries] = useState<SpeedPoint[]>([]);
  const tRef = useRef(0);
  const lastNet = useRef(0);
  const lastDisk = useRef(0);

  useEffect(() => {
    const un1 = listen<number>(`downloaded_bytes_${id}`, (ev) => {
      setDownloadedBytes(ev.payload);
    });


    const un2 = listen<SpeedAndRemaining>(`speed_and_remaining_${id}`, (ev) => {
      const net = Math.max(0, ev.payload.speed ?? 0);
      const disk = lastDisk.current;
      lastNet.current = net;

      setSr((prev) => ({ ...prev, ...ev.payload }));

      tRef.current += 1;
      setSeries((old) => {
        const next = [...old, { t: tRef.current, net, disk }];
        if (next.length > MAX_POINTS) next.shift();
        return next;
      });
    });


    const un3 = listen<number>(`disk_speed_${id}`, (ev) => {
      const disk = Math.max(0, ev.payload ?? 0);
      const net = lastNet.current;
      lastDisk.current = disk;

      setSr((prev) => ({ ...prev, diskSpeed: disk }));

      tRef.current += 1;
      setSeries((old) => {
        const next = [...old, { t: tRef.current, net, disk }];
        if (next.length > MAX_POINTS) next.shift();
        return next;
      });
    });

    const un4 = listen<number>(`wrote_bytes_${id}`, (ev) => {
      setWroteBytes(ev.payload);
    });

    return () => {
      un1.then((f) => f());
      un2.then((f) => f());
      un3.then((f) => f());
      un4.then((f) => f());
    };
  }, [id]);

  useEffect(() => {
    setDownloadedBytes(initialDownloadedBytes);
    setWroteBytes(initialDownloadedBytes);
  }, [initialDownloadedBytes]);

  const isWriting = status === Status.Writing;
  const isDownloading = status === Status.Downloading;

  const progress = useMemo(() => {
    const bytes = isWriting ? wroteBytes : downloadedBytes;
    return Math.round((Math.min(bytes, totalBytes) / Math.max(totalBytes, 1)) * 100);
  }, [isWriting, wroteBytes, downloadedBytes, totalBytes]);

  return (
    <div className="space-y-2">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <StatusPill status={status} />
          {isDownloading && (
            <>
              <span className="text-xs text-muted-foreground">•</span>
              <span className="inline-flex items-center gap-1 text-xs">
                <Download className="w-3 h-3" />
                {fmtThroughputKB(sr.speed)}
              </span>
              <span className="text-xs text-muted-foreground">/</span>
              <span className="inline-flex items-center gap-1 text-xs">
                <HardDrive className="w-3 h-3" />
                {fmtThroughputKB(sr.diskSpeed)}
              </span>
              <span className="text-xs text-muted-foreground">•</span>
              <span className="inline-flex items-center gap-1 text-xs">
                <Clock className="w-3 h-3" />
                {fmtRemaining(sr.remaining_time)}
              </span>
            </>
          )}
          {isWriting && (
            <span className="ml-2 text-xs text-purple-500">Writing to disk…</span>
          )}
        </div>
        <span className="text-xs text-muted-foreground">
          {((isWriting ? wroteBytes : downloadedBytes) / (1024 * 1024)).toFixed(1)} MB
        </span>
      </div>

      <div className="relative">
        <Progress
          value={progress}
          className={clsx(
            "h-2 bg-muted/30 overflow-hidden",
            "[&>div]:transition-all [&>div]:duration-300",
            isWriting
              ? "[&>div]:bg-gradient-to-r [&>div]:from-purple-500 [&>div]:to-purple-400"
              : "[&>div]:bg-gradient-to-r [&>div]:from-blue-500 [&>div]:to-cyan-400"
          )}
        />
        <div className="pointer-events-none absolute inset-x-0 -top-px h-px bg-gradient-to-r from-transparent via-white/30 to-transparent opacity-30" />
      </div>

      {isDownloading && <SpeedChart data={series} />}
    </div>
  );
}

export default memo(StatusAndSpeed);
