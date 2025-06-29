"use client"

import { memo, useEffect, useMemo, useState } from "react"
import { listen } from "@tauri-apps/api/event"
import { Progress } from "@/components/ui/progress"
import { Clock, Download, HardDrive } from "lucide-react"
import { Status } from "./types"

interface SpeedAndRemaining {
    speed: number
    remaining_time: number
    diskSpeed: number
}

function StatusAndSpeed({
    id,
    totalBytes,
    status,
    downloadedBytes: initialDownloadedBytes,
}: { id: number; totalBytes: number; status: Status; downloadedBytes: number }) {
    const [downloadedBytes, setDownloadedBytes] = useState(initialDownloadedBytes)
    const [wroteBytes, setWroteBytes] = useState(initialDownloadedBytes)
    const [speedAndRemaining, setSpeedAndRemaining] = useState<SpeedAndRemaining>({
        speed: 0,
        diskSpeed: 0,
        remaining_time: 0,
    })

    useEffect(() => {
        const unListen1 = listen<number>(`downloaded_bytes_${id}`, (ev) => {
            setDownloadedBytes(ev.payload)
        })

        const unListen2 = listen<SpeedAndRemaining>(`speed_and_remaining_${id}`, (ev) => {
            setSpeedAndRemaining((prev) => ({
                ...prev,
                ...ev.payload,
            }))
        })

        const unListen3 = listen<number>(`disk_speed_${id}`, (ev) => {
            setSpeedAndRemaining((prev) => ({ ...prev, diskSpeed: ev.payload }))
        })

        const unListen4 = listen<number>(`wrote_bytes_${id}`, (ev) => {
            setWroteBytes(ev.payload)
        })

        return () => {
            unListen1.then((fn) => fn())
            unListen2.then((fn) => fn())
            unListen3.then((fn) => fn())
            unListen4.then((fn) => fn())
        }
    }, [id])

    // Update local state when prop changes (for initial load)
    useEffect(() => {
        setDownloadedBytes(initialDownloadedBytes)
        setWroteBytes(initialDownloadedBytes)
    }, [initialDownloadedBytes])

    const downloadProgress = useMemo(() => {
        return Math.round((downloadedBytes / totalBytes) * 100)
    }, [downloadedBytes, totalBytes])

    const writeProgress = useMemo(() => {
        return Math.round((wroteBytes / totalBytes) * 100)
    }, [wroteBytes, totalBytes])

    const remainingTime = useMemo(() => {
        const second = Math.round(speedAndRemaining.remaining_time)
        const minute = Math.round(speedAndRemaining.remaining_time / 60)
        const hour = Math.round(speedAndRemaining.remaining_time / 60 / 60)
        const day = Math.round(speedAndRemaining.remaining_time / 60 / 60 / 24)

        if (day >= 1) {
            return day > 1 ? day + " days" : day + " day"
        }
        if (hour >= 1) {
            return hour > 1 ? hour + " hours" : hour + " hour"
        }
        if (minute >= 1) {
            return minute > 1 ? minute + " minutes" : minute + " minute"
        }

        return second > 1 ? second + " seconds" : second + " second"
    }, [speedAndRemaining.remaining_time])

    const speed = useMemo(() => {
        const kb = speedAndRemaining.speed;
        const mb = speedAndRemaining.speed / 1024;
        const gb = speedAndRemaining.speed / 1024 / 1024;

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
        const mb = speedAndRemaining.diskSpeed / 1024;
        const gb = speedAndRemaining.diskSpeed / 1024 / 1024;

        if (gb >= 1) {
            return gb.toFixed(1) + " GB/s";
        }
        if (mb >= 1) {
            return mb.toFixed(1) + " MB/s";
        }

        return kb.toFixed(1) + " KB/s";
    }, [speedAndRemaining.diskSpeed]);

    return (
        <div className="space-y-3">
            {/* Progress Section */}
            <div className="space-y-2">
                {status === Status.Writing ? (
                    // Show write progress when writing
                    <>
                        <div className="flex justify-between items-center">
                            <span className="text-sm font-medium">Writing: {writeProgress}%</span>
                            <span className="text-xs text-muted-foreground">{(wroteBytes / (1024 * 1024)).toFixed(1)} MB</span>
                        </div>
                        <Progress value={writeProgress} className="h-2 bg-muted/30 [&>div]:bg-purple-500" />
                    </>
                ) : (
                    // Show download progress for other statuses
                    <>
                        <div className="flex justify-between items-center">
                            <span className="text-sm font-medium">{downloadProgress}%</span>
                            <span className="text-xs text-muted-foreground">{(downloadedBytes / (1024 * 1024)).toFixed(1)} MB</span>
                        </div>
                        <Progress value={downloadProgress} className="h-2 bg-muted/30" />
                    </>
                )}
            </div>

            {/* Speed and Time Info */}
            {status === Status.Downloading && (
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

            {/* Show only disk speed when writing */}
            {status === Status.Writing && (
                <div className="flex items-center gap-3 text-xs">
                    <div className="flex items-center gap-1.5 px-2 py-1 rounded-md bg-muted/50">
                        <HardDrive className="w-3 h-3 text-purple-500" />
                        <span className="font-medium">{diskSpeed}</span>
                    </div>
                    <div className="flex items-center gap-1.5 px-2 py-1 rounded-md bg-purple-100 dark:bg-purple-900/30">
                        <span className="text-purple-600 dark:text-purple-400 font-medium">Writing to disk...</span>
                    </div>
                </div>
            )}
        </div>
    )
}

export default memo(StatusAndSpeed)
