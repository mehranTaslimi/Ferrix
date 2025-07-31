"use client";

import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import {
  FileText,
  ImageIcon,
  VideoIcon,
  Archive,
  Music,
  Code,
  FileIcon,
  X,
} from "lucide-react";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import { useDownloads } from "./download-context";

function getMimeTypeIcon(mimeType: string) {
  const iconClass = "w-4 h-4";
  if (mimeType.startsWith("image/")) return <ImageIcon className={iconClass} />;
  if (mimeType.startsWith("video/")) return <VideoIcon className={iconClass} />;
  if (mimeType.startsWith("audio/")) return <Music className={iconClass} />;
  if (
    mimeType.includes("zip") ||
    mimeType.includes("rar") ||
    mimeType.includes("tar")
  )
    return <Archive className={iconClass} />;
  if (
    mimeType.includes("text") ||
    mimeType.includes("json") ||
    mimeType.includes("xml")
  )
    return <Code className={iconClass} />;
  if (mimeType.includes("pdf")) return <FileText className={iconClass} />;
  return <FileIcon className={iconClass} />;
}

function getMimeTypeLabel(mimeType: string) {
  if (mimeType.startsWith("image/")) return "Images";
  if (mimeType.startsWith("video/")) return "Videos";
  if (mimeType.startsWith("audio/")) return "Audio";
  if (
    mimeType.includes("zip") ||
    mimeType.includes("rar") ||
    mimeType.includes("tar")
  )
    return "Archives";
  if (
    mimeType.includes("text") ||
    mimeType.includes("json") ||
    mimeType.includes("xml")
  )
    return "Documents";
  if (mimeType.includes("pdf")) return "PDFs";
  if (mimeType.includes("application")) return "Applications";
  return mimeType.split("/")[0] || "Other";
}

export default function AppSidebar() {
  const { downloads, selectedMimeType, setSelectedMimeType } = useDownloads();

  const mimeTypeStats = downloads.reduce((acc, download) => {
    const mimeType = download.content_type;
    if (!acc[mimeType]) {
      acc[mimeType] = 0;
    }
    acc[mimeType]++;
    return acc;
  }, {} as Record<string, number>);

  const groupedMimeTypes = Object.entries(mimeTypeStats).reduce(
    (acc, [mimeType, count]) => {
      const category = getMimeTypeLabel(mimeType);
      if (!acc[category]) {
        acc[category] = { count: 0, mimeTypes: [] };
      }
      acc[category].count += count;
      acc[category].mimeTypes.push(mimeType);
      return acc;
    },
    {} as Record<string, { count: number; mimeTypes: string[] }>
  );

  const totalDownloads = downloads.length;

  return (
    <Sidebar className="border-none">
      <SidebarContent>
        <SidebarGroup className="pt-14">
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  isActive={selectedMimeType === null}
                  onClick={() => setSelectedMimeType(null)}
                  className="w-full justify-between h-auto p-3"
                >
                  <div className="flex items-center gap-2">
                    <FileIcon className="w-4 h-4" />
                    <span>All Downloads</span>
                  </div>
                  <Badge
                    variant="outline"
                    className="ml-2 rounded-full h-6 w-6"
                  >
                    {totalDownloads}
                  </Badge>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarGroup>
          <SidebarGroupLabel>File Types</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              {Object.entries(groupedMimeTypes).map(
                ([category, { count, mimeTypes }]) => {
                  const isSelected = mimeTypes.some(
                    (mimeType) => selectedMimeType === mimeType
                  );
                  const primaryMimeType = mimeTypes[0];

                  return (
                    <SidebarMenuItem key={category}>
                      <SidebarMenuButton
                        isActive={isSelected}
                        onClick={() =>
                          setSelectedMimeType(
                            isSelected ? null : primaryMimeType
                          )
                        }
                        className="w-full justify-between h-auto p-2"
                      >
                        <div className="flex items-center gap-2">
                          {getMimeTypeIcon(primaryMimeType)}
                          <span>{category}</span>
                        </div>
                        <Badge
                          variant="outline"
                          className="h-6 w-6 ml-2 rounded-full"
                        >
                          {count}
                        </Badge>
                      </SidebarMenuButton>
                    </SidebarMenuItem>
                  );
                }
              )}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
  );
}
