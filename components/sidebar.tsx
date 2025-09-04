'use client';

import { FileIcon, Settings as SettingsIcon, X } from 'lucide-react';
import Link from 'next/link';
import { usePathname, useRouter } from 'next/navigation';

import { Badge } from '@/components/ui/badge';
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from '@/components/ui/sidebar';
import { cn } from '@/lib/utils';
import { iconForMime, labelForMime } from '@/utils/mime-utils';

import { useDownloads } from './download-context';

export default function AppSidebar() {
  const pathname = usePathname();
  const router = useRouter();
  const { downloads, selectedMimeType, setSelectedMimeType } = useDownloads();

  const mimeCounts = downloads.reduce(
    (acc, d) => {
      const mime = (d.content_type || 'application/octet-stream').toLowerCase();
      acc[mime] = (acc[mime] || 0) + 1;
      return acc;
    },
    {} as Record<string, number>,
  );

  const grouped = Object.entries(mimeCounts).reduce(
    (acc, [mime, count]) => {
      const label = labelForMime(mime);
      if (!acc[label]) acc[label] = { count: 0, mimeTypes: [] as string[] };
      acc[label].count += count;
      acc[label].mimeTypes.push(mime);
      return acc;
    },
    {} as Record<string, { count: number; mimeTypes: string[] }>,
  );

  const categories = Object.entries(grouped).sort((a, b) => {
    if (b[1].count !== a[1].count) return b[1].count - a[1].count;
    return a[0].localeCompare(b[0]);
  });

  const total = downloads.length;
  const isFiltered = selectedMimeType !== null;

  const goHomeThen = (fn?: () => void) => {
    if (pathname !== '/') router.push('/');
    fn?.();
  };

  return (
    <Sidebar className="border-none">
      <SidebarContent>
        <SidebarGroup className="pt-14">
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem className="flex items-center gap-1">
                <SidebarMenuButton
                  isActive={pathname === '/' && !isFiltered}
                  onClick={() => goHomeThen(() => setSelectedMimeType(null))}
                  className="h-auto w-full justify-between rounded-md p-3"
                >
                  <div className="flex items-center gap-2">
                    <FileIcon className="h-4 w-4" />
                    <span className="font-medium">All Downloads</span>

                    {isFiltered && (
                      <span
                        role="button"
                        tabIndex={0}
                        onClick={(e) => {
                          e.stopPropagation();
                          goHomeThen(() => setSelectedMimeType(null));
                        }}
                        onKeyDown={(e) => {
                          if (e.key === 'Enter' || e.key === ' ') {
                            e.preventDefault();
                            e.stopPropagation();
                            goHomeThen(() => setSelectedMimeType(null));
                          }
                        }}
                        className="text-muted-foreground hover:text-foreground ml-1 inline-flex items-center gap-1 text-xs"
                        title="Clear filter"
                        aria-label="Clear filter"
                      >
                        <X className="h-3 w-3" aria-hidden="true" />
                        Clear
                      </span>
                    )}
                  </div>

                  <Badge
                    variant="outline"
                    className="ml-2 h-6 w-6 rounded-full text-black dark:text-white"
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
                const isSelected = mimeTypes.some((mime) => selectedMimeType === mime);
                const primaryMime = mimeTypes[0];

                return (
                  <SidebarMenuItem key={label}>
                    <SidebarMenuButton
                      isActive={pathname === '/' && isSelected}
                      onClick={() =>
                        goHomeThen(() => setSelectedMimeType(isSelected ? null : primaryMime))
                      }
                      className={cn('h-auto w-full justify-between rounded-md p-2')}
                    >
                      <div className="flex items-center gap-2">
                        {iconForMime(primaryMime)}
                        <span>{label}</span>
                      </div>
                      <Badge
                        variant="outline"
                        className="ml-2 h-6 w-6 rounded-full text-black dark:text-white"
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

        <SidebarGroup>
          <SidebarGroupLabel>App</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  asChild
                  isActive={pathname.startsWith('/settings')}
                  className="h-auto w-full justify-between rounded-md p-3"
                >
                  <Link href="/settings">
                    <div className="flex items-center gap-2">
                      <SettingsIcon className="h-4 w-4" />
                      <span className="font-medium">Settings</span>
                    </div>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
  );
}
