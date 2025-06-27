"use client"

import type React from "react"
import { useState, useEffect } from "react"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Label } from "@/components/ui/label"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { invoke } from "@tauri-apps/api/core"
import { Plus, X } from "lucide-react"

interface DownloadFormModalProps {
    open: boolean
    onOpenChange: (open: boolean) => void
}

export default function DownloadFormModal({ open, onOpenChange }: DownloadFormModalProps) {
    const [url, setUrl] = useState("")
    const [chunk, setChunk] = useState("5")
    const [isLoading, setIsLoading] = useState(false)

    const handleSubmit = async () => {
        if (!url.trim()) return

        setIsLoading(true)
        try {
            await invoke("add_download_queue", {
                url: url.trim(),
                chunk: Number.parseInt(chunk) || 5,
            })
            // Reset form and close modal on success
            setUrl("")
            setChunk("5")
            onOpenChange(false)
        } catch (error) {
            console.error("Failed to add download:", error)
        } finally {
            setIsLoading(false)
        }
    }

    const handleKeyPress = (e: React.KeyboardEvent) => {
        if (e.key === "Enter" && !e.shiftKey) {
            e.preventDefault()
            handleSubmit()
        }
    }

    // Handle escape key
    useEffect(() => {
        const handleEscape = (e: KeyboardEvent) => {
            if (e.key === "Escape") {
                onOpenChange(false)
            }
        }

        if (open) {
            document.addEventListener("keydown", handleEscape)
            return () => document.removeEventListener("keydown", handleEscape)
        }
    }, [open, onOpenChange])

    if (!open) return null

    return (
        <div className="fixed inset-0 z-50 flex items-center justify-center">
            {/* Backdrop */}
            <div className="absolute inset-0 bg-black/50 backdrop-blur-sm" onClick={() => onOpenChange(false)} />

            {/* Modal Content */}
            <Card className="relative w-full max-w-md mx-4 shadow-lg">
                <CardHeader>
                    <div className="flex items-center justify-between">
                        <CardTitle className="flex items-center gap-2">
                            <Plus className="w-5 h-5" />
                            Add New Download
                        </CardTitle>
                        <Button variant="ghost" size="sm" onClick={() => onOpenChange(false)} className="h-8 w-8 p-0">
                            <X className="w-4 h-4" />
                        </Button>
                    </div>
                    <CardDescription>Enter a URL and configure download settings to start downloading a file</CardDescription>
                </CardHeader>

                <CardContent className="space-y-4">
                    <div className="space-y-2">
                        <Label htmlFor="url">Download URL *</Label>
                        <Input
                            id="url"
                            placeholder="https://example.com/file.mp4"
                            value={url}
                            onChange={(ev) => setUrl(ev.target.value)}
                            onKeyPress={handleKeyPress}
                            className="w-full"
                            autoFocus
                        />
                    </div>

                    <div className="space-y-2">
                        <Label htmlFor="chunk">Number of Chunks</Label>
                        <Input
                            id="chunk"
                            type="number"
                            min="1"
                            max="16"
                            placeholder="5"
                            value={chunk}
                            onChange={(ev) => setChunk(ev.target.value)}
                            onKeyPress={handleKeyPress}
                            className="w-full"
                        />
                        <p className="text-xs text-muted-foreground">
                            Higher chunk count may increase download speed but uses more resources (1-16)
                        </p>
                    </div>

                    <div className="flex gap-2 pt-4">
                        <Button variant="outline" onClick={() => onOpenChange(false)} disabled={isLoading} className="flex-1">
                            Cancel
                        </Button>
                        <Button onClick={handleSubmit} disabled={!url.trim() || isLoading} className="flex-1">
                            {isLoading ? "Adding..." : "Add Download"}
                        </Button>
                    </div>
                </CardContent>
            </Card>
        </div>
    )
}
