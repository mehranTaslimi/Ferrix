"use client";

import Link from "next/link";
import { usePathname, useRouter } from "next/navigation";
import { Badge } from "@/components/ui/badge";
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
import { iconForMime, labelForMime } from "@/utils/mime-utils";
import { FileIcon, X, Settings as SettingsIcon } from "lucide-react";
import { cn } from "@/lib/utils";

export default function AppSidebar() {
  const pathname = usePathname();
  const router = useRouter();
  const { downloads, selectedMimeType, setSelectedMimeType } = useDownloads();

  const mimeCounts = downloads.reduce((acc, d) => {
    const mime = (d.content_type || "application/octet-stream").toLowerCase();
    acc[mime] = (acc[mime] || 0) + 1;
    return acc;
  }, {} as Record<string, number>);

  const grouped = Object.entries(mimeCounts).reduce((acc, [mime, count]) => {
    const label = labelForMime(mime);
    if (!acc[label]) acc[label] = { count: 0, mimeTypes: [] as string[] };
    acc[label].count += count;
    acc[label].mimeTypes.push(mime);
    return acc;
  }, {} as Record<string, { count: number; mimeTypes: string[] }>);

  const categories = Object.entries(grouped).sort((a, b) => {
    if (b[1].count !== a[1].count) return b[1].count - a[1].count;
    return a[0].localeCompare(b[0]);
  });

  const total = downloads.length;
  const isFiltered = selectedMimeType !== null;

  const goHomeThen = (fn?: () => void) => {
    if (pathname !== "/") router.push("/");
    fn?.();
  };

  return (
    <Sidebar className="border-none">
      <SidebarContent>
        <SidebarGroup className="pt-14">
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  isActive={pathname === "/" && !isFiltered}
                  onClick={() => goHomeThen(() => setSelectedMimeType(null))}
                  className="w-full justify-between h-auto p-3 rounded-md"
                >
                  <div className="flex items-center gap-2">
                    <FileIcon className="w-4 h-4" />
                    <span className="font-medium">All Downloads</span>
                    {isFiltered && (
                      <span
                        onClick={(e) => {
                          e.stopPropagation();
                          goHomeThen(() => setSelectedMimeType(null));
                        }}
                        className="ml-1 inline-flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground"
                        title="Clear filter"
                      >
                        <X className="w-3 h-3" />
                        Clear
                      </span>
                    )}
                  </div>
                  <Badge
                    variant="outline"
                    className="ml-2 rounded-full h-6 w-6 dark:text-white text-black"
                  >
                    {total}
                  </Badge>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarGroup>
          {!!total && <SidebarGroupLabel>File Types</SidebarGroupLabel>}
          <SidebarGroupContent>
            <SidebarMenu>
              {categories.map(([label, { count, mimeTypes }]) => {
                const isSelected = mimeTypes.some(
                  (mime) => selectedMimeType === mime
                );
                const primaryMime = mimeTypes[0];

                return (
                  <SidebarMenuItem key={label}>
                    <SidebarMenuButton
                      isActive={pathname === "/" && isSelected}
                      onClick={() =>
                        goHomeThen(() =>
                          setSelectedMimeType(isSelected ? null : primaryMime)
                        )
                      }
                      className={cn("w-full justify-between h-auto p-2 rounded-md")}
                    >
                      <div className="flex items-center gap-2">
                        {iconForMime(primaryMime)}
                        <span>{label}</span>
                      </div>
                      <Badge
                        variant="outline"
                        className="h-6 w-6 ml-2 rounded-full dark:text-white text-black"
                      >
                        {count}
                      </Badge>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                );
              })}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        {/* <SidebarGroup>
          <SidebarGroupLabel>App</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  asChild
                  isActive={pathname.startsWith("/settings")}
                  className="w-full justify-between h-auto p-3 rounded-md"
                >
                  <Link href="/settings">
                    <div className="flex items-center gap-2">
                      <SettingsIcon className="w-4 h-4" />
                      <span className="font-medium">Settings</span>
                    </div>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup> */}
      </SidebarContent>
    </Sidebar>
  );
}
