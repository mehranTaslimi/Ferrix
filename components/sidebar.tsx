"use client"

import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { FileText, ImageIcon, VideoIcon, Archive, Music, Code, FileIcon } from "lucide-react"
import { useDownloads } from "./download-context"

function getMimeTypeIcon(mimeType: string) {
    const iconClass = "w-4 h-4"

    if (mimeType.startsWith("image/")) return <ImageIcon className={iconClass} />
    if (mimeType.startsWith("video/")) return <VideoIcon className={iconClass} />
    if (mimeType.startsWith("audio/")) return <Music className={iconClass} />
    if (mimeType.includes("zip") || mimeType.includes("rar") || mimeType.includes("tar"))
        return <Archive className={iconClass} />
    if (mimeType.includes("text") || mimeType.includes("json") || mimeType.includes("xml"))
        return <Code className={iconClass} />
    if (mimeType.includes("pdf")) return <FileText className={iconClass} />

    return <FileIcon className={iconClass} />
}

function getMimeTypeLabel(mimeType: string) {
    if (mimeType.startsWith("image/")) return "Images"
    if (mimeType.startsWith("video/")) return "Videos"
    if (mimeType.startsWith("audio/")) return "Audio"
    if (mimeType.includes("zip") || mimeType.includes("rar") || mimeType.includes("tar")) return "Archives"
    if (mimeType.includes("text") || mimeType.includes("json") || mimeType.includes("xml")) return "Documents"
    if (mimeType.includes("pdf")) return "PDFs"
    if (mimeType.includes("application")) return "Applications"

    return mimeType.split("/")[0] || "Other"
}

export default function Sidebar() {
    const { downloads, selectedMimeType, setSelectedMimeType } = useDownloads()

    // Get unique MIME types and their counts
    const mimeTypeStats = downloads.reduce(
        (acc, download) => {
            const mimeType = download.content_type
            if (!acc[mimeType]) {
                acc[mimeType] = 0
            }
            acc[mimeType]++
            return acc
        },
        {} as Record<string, number>,
    )

    // Group similar MIME types
    const groupedMimeTypes = Object.entries(mimeTypeStats).reduce(
        (acc, [mimeType, count]) => {
            const category = getMimeTypeLabel(mimeType)
            if (!acc[category]) {
                acc[category] = { count: 0, mimeTypes: [] }
            }
            acc[category].count += count
            acc[category].mimeTypes.push(mimeType)
            return acc
        },
        {} as Record<string, { count: number; mimeTypes: string[] }>,
    )

    const totalDownloads = downloads.length

    return (
        <div className="h-full p-6 pt-12 space-y-6">
            {/* All Downloads Button */}
            <div>
                <Button
                    variant={selectedMimeType === null ? "default" : "ghost"}
                    className="w-full justify-between h-auto p-3 font-normal"
                    onClick={() => setSelectedMimeType(null)}
                >
                    <div className="flex items-center gap-2">
                        <FileIcon className="w-4 h-4" />
                        <span>All Downloads</span>
                    </div>
                    <Badge variant="secondary" className="ml-2">
                        {totalDownloads}
                    </Badge>
                </Button>
            </div>

            {/* MIME Type Filters */}
            <div className="space-y-2">
                <h3 className="text-sm font-medium text-muted-foreground uppercase tracking-wide">File Types</h3>
                <div className="space-y-1">
                    {Object.entries(groupedMimeTypes).map(([category, { count, mimeTypes }]) => {
                        const isSelected = mimeTypes.some((mimeType) => selectedMimeType === mimeType)
                        const primaryMimeType = mimeTypes[0]

                        return (
                            <Button
                                key={category}
                                variant={isSelected ? "default" : "ghost"}
                                className="w-full justify-between h-auto p-3 font-normal"
                                onClick={() => setSelectedMimeType(isSelected ? null : primaryMimeType)}
                            >
                                <div className="flex items-center gap-2">
                                    {getMimeTypeIcon(primaryMimeType)}
                                    <span>{category}</span>
                                </div>
                                <Badge variant="secondary" className="ml-2">
                                    {count}
                                </Badge>
                            </Button>
                        )
                    })}
                </div>
            </div>

            {/* Active Filter Info */}
            {selectedMimeType && (
                <div className="p-3 rounded-lg bg-muted/50 border border-muted">
                    <div className="flex items-center gap-2 mb-1">
                        {getMimeTypeIcon(selectedMimeType)}
                        <span className="text-sm font-medium">Active Filter</span>
                    </div>
                    <p className="text-xs text-muted-foreground">Showing {getMimeTypeLabel(selectedMimeType).toLowerCase()}</p>
                    <Button
                        variant="outline"
                        size="sm"
                        className="mt-2 h-7 text-xs bg-transparent"
                        onClick={() => setSelectedMimeType(null)}
                    >
                        Clear Filter
                    </Button>
                </div>
            )}
        </div>
    )
}
