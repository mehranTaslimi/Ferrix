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

        return () => {
            unListen1.then((fn) => fn())
            unListen2.then((fn) => fn())
            unListen3.then((fn) => fn())
        }
    }, [id])

    // Update local state when prop changes (for initial load)
    useEffect(() => {
        setDownloadedBytes(initialDownloadedBytes)
    }, [initialDownloadedBytes])

    const progress = useMemo(() => {
        return Math.round((downloadedBytes / totalBytes) * 100)
    }, [downloadedBytes, totalBytes])

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
        const kb = Math.round(speedAndRemaining.speed)
        const mb = Math.round(speedAndRemaining.speed / 1024)
        const gb = Math.round(speedAndRemaining.speed / 1024 / 1024)

        if (gb >= 1) {
            return gb + " GB/s"
        }
        if (mb >= 1) {
            return mb + " MB/s"
        }

        return kb + " KB/s"
    }, [speedAndRemaining.speed])

    const diskSpeed = useMemo(() => {
        const kb = Math.round(speedAndRemaining.diskSpeed)
        const mb = Math.round(speedAndRemaining.diskSpeed / 1024)
        const gb = Math.round(speedAndRemaining.diskSpeed / 1024 / 1024)

        if (gb >= 1) {
            return gb + " GB/s"
        }
        if (mb >= 1) {
            return mb + " MB/s"
        }

        return kb + " KB/s"
    }, [speedAndRemaining.diskSpeed])

    return (
        <div className="space-y-3">
            {/* Progress Section */}
            <div className="space-y-2">
                <div className="flex justify-between items-center">
                    <span className="text-sm font-medium">{progress}%</span>
                    <span className="text-xs text-muted-foreground">{(downloadedBytes / (1024 * 1024)).toFixed(1)} MB</span>
                </div>
                <Progress value={progress} className="h-2 bg-muted/30" />
            </div>

            {/* Speed and Time Info - Only show when downloading */}
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
        </div>
    )
}

export default memo(StatusAndSpeed)
