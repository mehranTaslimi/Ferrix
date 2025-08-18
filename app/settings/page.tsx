"use client";

import { useEffect, useState } from "react";
import { useTheme } from "next-themes";

import {
    Accordion,
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
} from "@/components/ui/accordion";

import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";

import { Label } from "@/components/ui/label";
import { Monitor, Moon, Store, Sun } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";

export default function SettingsPage() {
    const { theme, setTheme } = useTheme();

    return (
        <div className="m-4">
            <header>
                <h1 className="text-2xl font-bold tracking-tight">Settings</h1>
                <p className="mt-2 text-muted-foreground text-sm">
                    Customize how Ferrix looks and behaves. Appearance, plugins, and
                    download builder options can be managed here. More settings are on the
                    way — stay tuned!
                </p>
            </header>
            <Accordion type="single" collapsible className="w-full" defaultValue="appearance">
                <AccordionItem value="appearance">
                    <AccordionTrigger>Appearance</AccordionTrigger>
                    <AccordionContent className="flex flex-col gap-4">
                        <p className="text-sm text-muted-foreground">
                            Choose how Ferrix looks. Switch between light, dark, or follow your system preference.
                        </p>
                        <div className="space-y-2">
                            <Label htmlFor="theme">Theme</Label>
                            <Select value={theme ?? "system"} onValueChange={setTheme}>
                                <SelectTrigger id="theme" className="w-[220px]">
                                    <SelectValue placeholder="Select theme" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem value="light">
                                        <div className="flex items-center gap-2">
                                            <Sun className="h-4 w-4" />
                                            <span>Light</span>
                                        </div>
                                    </SelectItem>
                                    <SelectItem value="dark">
                                        <div className="flex items-center gap-2">
                                            <Moon className="h-4 w-4" />
                                            <span>Dark</span>
                                        </div>
                                    </SelectItem>
                                    <SelectItem value="system">
                                        <div className="flex items-center gap-2">
                                            <Monitor className="h-4 w-4" />
                                            <span>System</span>
                                        </div>
                                    </SelectItem>
                                </SelectContent>
                            </Select>
                        </div>
                    </AccordionContent>
                </AccordionItem>
                <AccordionItem value="plugins">
                    <AccordionTrigger>
                        <div className="flex items-center gap-2">
                            <span>Plugins</span>
                            <Badge variant="secondary" className="uppercase">Coming soon</Badge>
                        </div>
                    </AccordionTrigger>
                    <AccordionContent className="flex flex-col gap-4">
                        <p className="text-sm text-muted-foreground">
                            Extend Ferrix with community-made integrations (sites, auth flows, post-processing, and more).
                            A plugin store and developer docs are on the way.
                        </p>

                        <div className="flex items-center gap-3">
                            <Button variant="default" disabled>
                                <Store className="mr-2 h-4 w-4" />
                                Open Plugin Store
                            </Button>
                            <span className="text-xs text-muted-foreground">(Coming soon…)</span>
                        </div>
                    </AccordionContent>
                </AccordionItem>
                <AccordionItem value="download-builder">
                    <AccordionTrigger>
                        <div className="flex items-center gap-2">
                            <span>Default Download Builder</span>
                            <Badge variant="secondary" className="uppercase">Coming soon</Badge>
                        </div>
                    </AccordionTrigger>
                    <AccordionContent className="flex flex-col gap-4">
                        <p className="text-sm text-muted-foreground">
                            Configure how new downloads are created by default—chunk size, concurrency, verification strategy,
                            user-agent, proxy profile, and automatic post-actions. Presets will let you switch profiles quickly.
                        </p>

                        <div className="space-y-2">
                            <Label htmlFor="builder-preset">Preset</Label>
                            <Select disabled value="default">
                                <SelectTrigger id="builder-preset" className="w-[260px]">
                                    <SelectValue placeholder="Select preset" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem value="default">Default</SelectItem>
                                    <SelectItem value="high-throughput">High Throughput</SelectItem>
                                    <SelectItem value="low-cpu">Low CPU</SelectItem>
                                    <SelectItem value="satellite">High Latency</SelectItem>
                                </SelectContent>
                            </Select>
                            <span className="text-xs text-muted-foreground">Coming soon…</span>
                        </div>
                    </AccordionContent>
                </AccordionItem>
            </Accordion>
        </div>
    );
}