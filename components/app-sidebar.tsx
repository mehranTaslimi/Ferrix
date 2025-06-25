"use client";
import {
  Download,
  ImageIcon,
  Video,
  Folder,
  Plus,
  Moon,
  Sun,
} from "lucide-react";
import { useTheme } from "next-themes";

import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarRail,
} from "@/components/ui/sidebar";
import { Button } from "@/components/ui/button";

export const activeCategory = ["all", "video", "image", "personal"] as const;
export type ActiveCategoryType = (typeof activeCategory)[number];
interface DownloadItem {
  file_name: string;
  content_type: string;
  extension: string;
}

interface AppSidebarProps {
  activeCategory: ActiveCategoryType;
  onCategoryChange: (category: ActiveCategoryType) => void;
  downloads: Record<number, DownloadItem>
}

export function AppSidebar({
  activeCategory,
  onCategoryChange,
  downloads,
}: AppSidebarProps) {
  const { setTheme, theme } = useTheme();

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

  const getCategoryCount = (categoryId: ActiveCategoryType): number => {
    switch (categoryId) {
      case "all":
        return Object.values(downloads).length;
      case "video":
        return Object.values(downloads).filter((item) => getFileTypeFromItem(item) === "video")
          .length;
      case "image":
        return Object.values(downloads).filter((item) => getFileTypeFromItem(item) === "image")
          .length;
      case "personal":
        return Object.values(downloads).filter((item) =>
          item.file_name.toLowerCase().includes("personal")
        ).length;
      default:
        return 0;
    }
  };

  const defaultCategories = [
    {
      title: "All Downloads",
      icon: Download,
      id: "all" as const,
      count: getCategoryCount("all"),
    },
    {
      title: "Video",
      icon: Video,
      id: "video" as const,
      count: getCategoryCount("video"),
    },
    {
      title: "Image",
      icon: ImageIcon,
      id: "image" as const,
      count: getCategoryCount("image"),
    },
  ];

  const personalCategories = [
    {
      title: "Personal",
      icon: Folder,
      id: "personal" as const,
      count: getCategoryCount("personal"),
    },
  ];

  return (
    <Sidebar className="frosted-glass border-r border-gray-300 dark:border-white/10 h-screen sticky">
      <SidebarHeader className="frosted-glass border-b border-gray-300 dark:border-white/10">
        <div className="flex items-center gap-3 px-4 py-4">
          <div className="relative flex h-9 w-9 items-center justify-center rounded-xl glass-morphism">
            <div className="absolute inset-0 rounded-xl bg-gradient-to-br from-gray-700/20 to-gray-700/10 dark:from-white/20 dark:to-white/5"></div>
            <Download className="relative h-4 w-4 text-gray-900 dark:text-foreground/80" />
          </div>
          <div className="flex flex-col">
            <span className="text-sm font-semibold text-gray-900 dark:text-foreground">
              Download Manager
            </span>
            <span className="text-xs text-gray-800 dark:text-muted-foreground">
              Manage your files
            </span>
          </div>
        </div>
      </SidebarHeader>

      <SidebarContent className="bg-transparent">
        <SidebarGroup>
          <SidebarGroupLabel className="text-xs font-medium text-gray-800 dark:text-muted-foreground px-3 py-2">
            Categories
          </SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu className="space-y-1 px-2">
              {defaultCategories.map((item) => (
                <SidebarMenuItem key={item.id}>
                  <SidebarMenuButton
                    onClick={() => onCategoryChange(item.id)}
                    isActive={activeCategory === item.id}
                    className={`group relative overflow-hidden rounded-xl transition-all duration-300 glass-morphism-hover ${activeCategory === item.id ? "glass-morphism-active" : ""
                      }`}
                  >
                    <item.icon className="h-4 w-4 text-gray-900 dark:text-foreground/70" />
                    <span className="flex-1 text-gray-900 dark:text-foreground/90">
                      {item.title}
                    </span>
                    <span className="rounded-full glass-morphism px-2 py-0.5 text-xs text-gray-900 dark:text-foreground/70 border border-gray-200 dark:border-white/10">
                      {item.count}
                    </span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarGroup>
          <SidebarGroupLabel className="flex items-center justify-between text-xs font-medium text-gray-800 dark:text-muted-foreground px-3 py-2">
            Personal Categories
            <Button
              variant="ghost"
              size="sm"
              className="h-5 w-5 p-0 glass-morphism-hover rounded-lg"
            >
              <Plus className="h-3 w-3 text-gray-900 dark:text-foreground" />
            </Button>
          </SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu className="space-y-1 px-2">
              {personalCategories.map((item) => (
                <SidebarMenuItem key={item.id}>
                  <SidebarMenuButton
                    onClick={() => onCategoryChange(item.id)}
                    isActive={activeCategory === item.id}
                    className={`group relative overflow-hidden rounded-xl transition-all duration-300 glass-morphism-hover ${activeCategory === item.id ? "glass-morphism-active" : ""
                      }`}
                  >
                    <item.icon className="h-4 w-4 text-gray-900 dark:text-foreground/70" />
                    <span className="flex-1 text-gray-900 dark:text-foreground/90">
                      {item.title}
                    </span>
                    <span className="rounded-full glass-morphism px-2 py-0.5 text-xs text-gray-900 dark:text-foreground/70 border border-gray-200 dark:border-white/10">
                      {item.count}
                    </span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>

      <SidebarFooter className="frosted-glass border-t border-gray-300 dark:border-white/10">
        <div className="px-4 py-3">
          <Button
            variant="ghost"
            size="sm"
            onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
            className="w-full justify-start glass-morphism-hover rounded-xl h-9 text-gray-900 dark:text-foreground border border-gray-300 dark:border-white/10"
          >
            {theme === "dark" ? (
              <>
                <Sun className="mr-2 h-4 w-4" />
                <span>Light Mode</span>
              </>
            ) : (
              <>
                <Moon className="mr-2 h-4 w-4" />
                <span>Dark Mode</span>
              </>
            )}
          </Button>
        </div>
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  );
}
