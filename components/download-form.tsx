"use client"

import type React from "react"

import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Label } from "@/components/ui/label"
import { useState } from "react"
import { invoke } from "@tauri-apps/api/core"
import { Plus } from "lucide-react"

export default function DownloadForm() {
    const [value, setValue] = useState(
        "https://caspian19.asset.aparat.com/aparat-video/9dcfe3d6b9a30b4b3706abb31cc2151d62651986-360p.mp4?wmsAuthSign=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ0b2tlbiI6ImQ0NzVhZGFiNjM3ZTIwOTVkMmY1ZjYzMzY2ODU4ZjBmIiwiZXhwIjoxNzUwNTQ4NDMzLCJpc3MiOiJTYWJhIElkZWEgR1NJRyJ9.GOxhbEA-eYErdeJQB6vEJGkbeS2K1W0REaebE-daEwQ",
    )
    const [isLoading, setIsLoading] = useState(false)

    const handleSubmit = async () => {
        if (!value.trim()) return

        setIsLoading(true)
        try {
            await invoke("add_download_queue", {
                url: value,
                chunk: 5,
            })
            setValue("")
        } catch (error) {
            console.error("Failed to add download:", error)
        } finally {
            setIsLoading(false)
        }
    }

    const handleKeyPress = (e: React.KeyboardEvent) => {
        if (e.key === "Enter") {
            handleSubmit()
        }
    }

    return (
        <Card className="mb-6">
            <CardHeader>
                <CardTitle className="flex items-center gap-2">
                    <Plus className="w-5 h-5" />
                    Add New Download
                </CardTitle>
                <CardDescription>Enter a URL to start downloading a file</CardDescription>
            </CardHeader>
            <CardContent>
                <div className="space-y-4">
                    <div className="space-y-2">
                        <Label htmlFor="url">Download URL</Label>
                        <Input
                            id="url"
                            placeholder="https://example.com/file.mp4"
                            value={value}
                            onChange={(ev) => setValue(ev.target.value)}
                            onKeyPress={handleKeyPress}
                        />
                    </div>
                    <Button onClick={handleSubmit} disabled={!value.trim() || isLoading} className="w-full">
                        {isLoading ? "Adding..." : "Add Download"}
                    </Button>
                </div>
            </CardContent>
        </Card>
    )
}
