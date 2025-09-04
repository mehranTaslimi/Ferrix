'use client';

import { listen } from '@tauri-apps/api/event';
import { AnimatePresence, motion } from 'framer-motion';
import { DownloadIcon } from 'lucide-react';
import { useEffect, useMemo, useRef, useState } from 'react';
import { toast } from 'sonner';

import DownloadBar from '@/components/download-bar';
import { EmptyDownloadsIntro } from '@/components/empty-download-intro';

import { useDownloads } from '../components/download-context';
import DownloadItem from '../components/download-item';
import DownloadSettingSheet from '../components/download-setting/download-setting-sheet';
import { Status } from '../components/types';

const statusRank = (s: Status) => {
  switch (s) {
    case Status.Downloading:
      return 0;
    case Status.Queued:
      return 1;
    default:
      return 2;
  }
};

export default function Page() {
  const { filteredDownloads, isLoading } = useDownloads();
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [url, setUrl] = useState('');

  const prevStatus = useRef<Map<number, Status>>(new Map());
  const [justPromotedId, setJustPromotedId] = useState<number | null>(null);

  useEffect(() => {
    const unlisten = listen<string>('error', (ev) => {
      toast.error('Error', { description: ev.payload });
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  useEffect(() => {
    for (const d of filteredDownloads) {
      const prev = prevStatus.current.get(d.id);
      if (prev && prev !== Status.Downloading && d.status === Status.Downloading) {
        setJustPromotedId(d.id);
        // const t = setTimeout(() => {
        //   setJustPromotedId((x) => (x === d.id ? null : x));
        // }, 1200);
      }
    }
    for (const d of filteredDownloads) {
      prevStatus.current.set(d.id, d.status);
    }
  }, [filteredDownloads]);

  const ordered = useMemo(() => {
    return [...filteredDownloads].sort((a, b) => {
      const rA = statusRank(a.status);
      const rB = statusRank(b.status);
      if (rA !== rB) return rA - rB;

      if (a.status === Status.Downloading && b.status === Status.Downloading) {
        return new Date(b.modified_at).getTime() - new Date(a.modified_at).getTime();
      }

      return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
    });
  }, [filteredDownloads]);

  if (isLoading) {
    return (
      <div className="container mx-auto max-w-4xl">
        <div className="bg-card sticky top-0 z-20 mb-3 h-[45px]"></div>
        <div className="flex h-64 items-center justify-center">
          <div className="text-center">
            <DownloadIcon className="mx-auto mb-2 h-8 w-8 animate-pulse" />
            <p className="text-muted-foreground">Loading downloads...</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="container mx-auto max-w-5xl">
      <div className="bg-background sticky top-0 z-20 mb-3">
        <DownloadBar setUrl={setUrl} url={url} setIsModalOpen={setIsModalOpen} />
      </div>

      <div className="space-y-4 px-3">
        {ordered.length === 0 ? (
          <EmptyDownloadsIntro />
        ) : (
          <div className="grid grid-cols-[repeat(auto-fill,_minmax(500px,_1fr))] gap-2">
            <AnimatePresence initial={false}>
              {ordered.map((item) => {
                const highlighted = justPromotedId === item.id;
                return (
                  <motion.div
                    key={item.id}
                    layout
                    initial={highlighted ? { opacity: 0, y: 16, scale: 0.98 } : false}
                    animate={{ opacity: 1, y: 0, scale: 1 }}
                    exit={{ opacity: 0, y: -8 }}
                    transition={{
                      type: 'spring',
                      stiffness: 420,
                      damping: 34,
                      mass: 0.6,
                    }}
                    className={highlighted ? 'rounded-xl ring-2 ring-blue-500/40' : ''}
                  >
                    <DownloadItem download={item} />
                  </motion.div>
                );
              })}
            </AnimatePresence>
          </div>
        )}
      </div>

      <DownloadSettingSheet
        setUrl={setUrl}
        url={url}
        open={isModalOpen}
        onOpenChange={setIsModalOpen}
      />
    </div>
  );
}
