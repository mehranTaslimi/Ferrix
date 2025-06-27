"use client"

import { memo } from "react"
import { Pause, Play, DownloadIcon, FileText, ImageIcon, Archive } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import { invoke } from "@tauri-apps/api/core"
import StatusAndSpeed from "./status-and-speed"
import { type DownloadType, Status, Extension } from "./types"

function getFileIcon(extension: Extension) {
    const iconClass = "w-5 h-5"
    switch (extension) {
        case Extension.Mp4:
            return <DownloadIcon className={iconClass} />
        case Extension.Jpg:
            return <ImageIcon className={iconClass} />
        case Extension.Rar:
            return <Archive className={iconClass} />
        default:
            return <FileText className={iconClass} />
    }
}

function getStatusColor(status: Status) {
    switch (status) {
        case Status.Downloading:
            return "bg-blue-500"
        case Status.Completed:
            return "bg-green-500"
        case Status.Failed:
            return "bg-red-500"
        case Status.Paused:
            return "bg-yellow-500"
        case Status.Queued:
            return "bg-gray-500"
        default:
            return "bg-gray-500"
    }
}

function DownloadItem({ download }: { download: DownloadType }) {
    const handleToggleDownload = async () => {
        if (download.status === Status.Paused) {
            await invoke("resume_download", { id: download.id })
        } else if (download.status === Status.Downloading) {
            await invoke("pause_download", { id: download.id })
        }
    }

    return (
        <Card className="group hover:shadow-lg transition-all duration-200 border-0 shadow-sm bg-gradient-to-br from-card to-card/50">
            <CardContent className="p-6">
                <div className="space-y-4">
                    {/* Header */}
                    <div className="flex items-start justify-between gap-3">
                        <div className="flex items-center gap-3 min-w-0 flex-1">
                            <div className="flex-shrink-0 p-2 rounded-lg bg-muted/50">{getFileIcon(download.extension)}</div>
                            <div className="min-w-0 flex-1">
                                <h3 className="font-semibold text-sm leading-tight truncate mb-1" title={download.file_name}>
                                    {download.file_name}
                                </h3>
                                <p className="text-xs text-muted-foreground">{(download.total_bytes / (1024 * 1024)).toFixed(1)} MB</p>
                            </div>
                        </div>

                        {/* Status indicator */}
                        <div className="flex items-center gap-2">
                            <div className={`w-2 h-2 rounded-full ${getStatusColor(download.status)}`} />
                            <span className="text-xs font-medium text-muted-foreground uppercase tracking-wide">
                                {download.status}
                            </span>
                        </div>
                    </div>

                    {/* Progress and Speed Section - Handled by StatusAndSpeed component */}
                    <StatusAndSpeed
                        id={download.id}
                        totalBytes={download.total_bytes}
                        downloadedBytes={download.downloaded_bytes}
                        status={download.status}
                    />

                    {/* Action Button */}
                    {download.status !== Status.Completed && download.status !== Status.Failed && (
                        <div className="pt-2">
                            <Button
                                onClick={handleToggleDownload}
                                variant="outline"
                                size="sm"
                                className="w-full h-9 font-medium transition-all duration-200 hover:scale-[1.02] bg-transparent"
                            >
                                {download.status === Status.Paused || download.status === Status.Queued ? (
                                    <>
                                        <Play className="w-4 h-4 mr-2" />
                                        Resume
                                    </>
                                ) : (
                                    <>
                                        <Pause className="w-4 h-4 mr-2" />
                                        Pause
                                    </>
                                )}
                            </Button>
                        </div>
                    )}
                </div>
            </CardContent>
        </Card>
    )
}

export default memo(DownloadItem)
