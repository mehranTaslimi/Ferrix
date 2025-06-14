"use client";

import * as React from "react";
import {
  KeyboardSensor,
  MouseSensor,
  TouchSensor,
  useSensor,
  useSensors,
  type UniqueIdentifier,
} from "@dnd-kit/core";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useForm } from "react-hook-form";
import { Plus, Search } from "lucide-react";

import { ActiveCategoryType, AppSidebar } from "@/components/app-sidebar";
import { DownloadCard } from "@/components/downoad-card";
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from "@/components/ui/sidebar";
import { DownLoadFormDialog } from "./download-form-dialog";

interface DownloadItem {
  id: number;
  file_name: string;
  file_path: string;
  url: string;
  status: "queued" | "downloading" | "completed" | "failed";
  total_bytes: number;
  downloaded_bytes: number;
  extension: string;
  content_type: string;
  created_at: string;
  chunk_count?: number;
  speed?: number;
}

const getCategoryTitle = (category: string) => {
  switch (category) {
    case "all":
      return "All Downloads";
    case "videos":
      return "Videos";
    case "images":
      return "Images";
    case "work":
      return "Work Documents";
    case "personal":
      return "Personal";
    default:
      return "Downloads";
  }
};
const getFileTypeFromItem = (item: DownloadItem): string => {
  if (
    item.content_type.startsWith("video/") ||
    ["mp4", "avi", "mkv", "mov", "wmv"].includes(item.extension.toLowerCase())
  ) {
    return "video";
  }
  if (
    item.content_type.startsWith("image/") ||
    ["jpg", "jpeg", "png", "gif", "webp", "svg"].includes(
      item.extension.toLowerCase()
    )
  ) {
    return "image";
  }
  return "other";
};

export function DownloadDashboard({
  data: initialData,
  onAddDownload,
}: {
  data: DownloadItem[];
  onAddDownload: (url: string, chunkCount: number) => Promise<void>;
}) {
  const [data, setData] = React.useState(() => initialData);
  const [activeCategory, setActiveCategory] =
    React.useState<ActiveCategoryType>("all");
  const [searchQuery, setSearchQuery] = React.useState("");
  const [isDownloadDialogOpen, setIsDownloadDialogOpen] = React.useState(false);

  const sensors = useSensors(
    useSensor(MouseSensor, {}),
    useSensor(TouchSensor, {}),
    useSensor(KeyboardSensor, {})
  );

  const form = useForm({
    defaultValues: {
      downloadUrl:
        "https://wallpaperswide.com/download/most_beautiful_mountain_scenery-wallpaper-3840x2160.jpg",
      chunkCount: 6,
    },
  });

  // Update data when initialData changes
  React.useEffect(() => {
    setData(initialData);
  }, [initialData]);

  const dataIds = React.useMemo<UniqueIdentifier[]>(
    () => data?.map(({ id }) => id) || [],
    [data]
  );

  const filteredDownloads = data.filter((item) => {
    const matchesSearch = item.file_name
      .toLowerCase()
      .includes(searchQuery.toLowerCase());
    const fileType = getFileTypeFromItem(item);
    const matchesCategory =
      activeCategory === "all" ||
      (activeCategory === "video" && fileType === "video") ||
      (activeCategory === "image" && fileType === "image") ||
      (activeCategory === "personal" &&
        item.file_name.toLowerCase().includes("personal"));
    return matchesSearch && matchesCategory;
  });
  return (
    <SidebarProvider>
      <AppSidebar
        activeCategory={activeCategory}
        onCategoryChange={(category) => setActiveCategory(category)}
        downloads={data}
      />
      <SidebarInset className="relative light-bg">
        {/* Liquid glass background elements */}
        <div className="absolute inset-0 overflow-hidden">
          <div className="absolute top-1/4 left-1/4 w-96 h-96 bg-foreground/10 dark:bg-foreground/5 rounded-full blur-3xl "></div>
          <div
            className="absolute bottom-1/4 right-1/4 w-80 h-80 bg-foreground/8 dark:bg-foreground/3 rounded-full blur-3xl "
            style={{ animationDelay: "10s" }}
          ></div>
        </div>

        {/* Header */}
        <header
          data-tauri-drag-region
          className="sticky top-0 z-10 flex h-16 shrink-0 px-2 items-center gap-2 border-b border-border/50 frosted-glass"
        >
          <SidebarTrigger className="p-4 ml-2 glass-morphism-active glass-morphism-hover rounded-lg lg:hidden" />
          <div className="flex flex-1 items-center gap-4">
            <div className="relative flex-1 max-w-md">
              <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
              <Input
                placeholder="Search downloads..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="pl-9 border-border/50 frosted-glass focus:glass-morphism-active text-foreground placeholder:text-muted-foreground"
              />
            </div>
            <Button
              onClick={() => setIsDownloadDialogOpen(true)}
              className="glass-morphism-active hover:glass-morphism-hover text-foreground border border-border/50 font-medium"
            >
              <Plus className="mr-2 h-4 w-4" />
              New Download
            </Button>
          </div>
        </header>

        {/* Main Content */}
        {isDownloadDialogOpen && (
          <DownLoadFormDialog
            onAddDownload={onAddDownload}
            setIsDialogOpen={setIsDownloadDialogOpen}
            isOpen={true}
          />
        )}
        <div className="relative flex-1 p-6">
          <div className="mb-6">
            <h1 className="text-2xl font-semibold text-foreground">
              {getCategoryTitle(activeCategory)}
            </h1>
            <p className="text-muted-foreground">
              {filteredDownloads.length}{" "}
              {filteredDownloads.length === 1 ? "item" : "items"}
            </p>
          </div>

          {filteredDownloads.length === 0 ? (
            <div className="flex flex-col items-center justify-center py-12 text-center">
              <div className="rounded-full glass-morphism p-6">
                <Search className="h-12 w-12 text-muted-foreground" />
              </div>
              <h3 className="mt-4 text-lg font-medium text-foreground">
                No downloads found
              </h3>
              <p className="text-muted-foreground">
                {searchQuery
                  ? "Try adjusting your search terms"
                  : "Start by adding a new download"}
              </p>
            </div>
          ) : (
            <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
              {filteredDownloads.map((item) => (
                <DownloadCard
                  key={item.id}
                  item={item}
                  onAction={onAddDownload}
                />
              ))}
            </div>
          )}
        </div>
      </SidebarInset>
    </SidebarProvider>
  );
}
