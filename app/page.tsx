"use client";

import { useEffect, useMemo, useRef, useState } from "react";
import DownloadItem from "../components/download-item";
import DownloadSettingSheet from "../components/download-setting/download-setting-sheet";
import { useDownloads } from "../components/download-context";
import { Card, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { DownloadIcon, Plus } from "lucide-react";
import DownloadBar from "@/components/download-bar";
import { listen } from "@tauri-apps/api/event";
import { toast } from "sonner";
import { AnimatePresence, motion } from "framer-motion";
import { Status } from "../components/types";


const statusRank = (s: Status) => {
  switch (s) {
    case Status.Downloading: return 0;
    case Status.Queued: return 1;
    default: return 2;
  }
};

export default function Page() {
  const { filteredDownloads, selectedMimeType, isLoading } = useDownloads();
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [url, setUrl] = useState("");


  const prevStatus = useRef<Map<number, Status>>(new Map());
  const [justPromotedId, setJustPromotedId] = useState<number | null>(null);

  useEffect(() => {
    const unlisten = listen<string>("error", (ev) => {
      toast.error("Error", { description: ev.payload });
    });
    return () => { unlisten.then((fn) => fn()); };
  }, []);


  useEffect(() => {
    for (const d of filteredDownloads) {
      const prev = prevStatus.current.get(d.id);
      if (prev && prev !== Status.Downloading && d.status === Status.Downloading) {
        setJustPromotedId(d.id);
        const t = setTimeout(() => {
          setJustPromotedId((x) => (x === d.id ? null : x));
        }, 1200);


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
        <div className="flex items-center justify-center h-64">
          <div className="text-center">
            <DownloadIcon className="w-8 h-8 mx-auto mb-2 animate-pulse" />
            <p className="text-muted-foreground">Loading downloads...</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="container mx-auto max-w-5xl">
      {/* small nit: z-20 instead of z-2 so it really stays on top */}
      <div className="mb-3 sticky top-0 z-20 bg-background">
        <DownloadBar
          setUrl={setUrl}
          url={url}
          filteredDownloads={ordered}
          selectedMimeType={selectedMimeType}
          setIsModalOpen={setIsModalOpen}
        />
      </div>

      <div className="space-y-4 px-3">
        {ordered.length === 0 ? (
          <Card>
            <CardContent className="flex flex-col items-center justify-center py-12">
              <DownloadIcon className="w-12 h-12 text-muted-foreground mb-4" />
              <h3 className="text-lg font-medium mb-2">
                {selectedMimeType ? "No downloads match this filter" : "No downloads yet"}
              </h3>
              <p className="text-muted-foreground text-center mb-4">
                {selectedMimeType
                  ? "Try selecting a different file type or clear the filter"
                  : 'Click the "New Download" button to get started'}
              </p>
              <Button onClick={() => setIsModalOpen(true)} variant="outline">
                <Plus className="w-4 h-4 mr-2" />
                Add Your First Download
              </Button>
            </CardContent>
          </Card>
        ) : (

          <div className="grid gap-2 grid-cols-[repeat(auto-fill,_minmax(500px,_1fr))]">
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
                    transition={{ type: "spring", stiffness: 420, damping: 34, mass: 0.6 }}
                    className={highlighted ? "ring-2 ring-blue-500/40 rounded-xl" : ""}
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
