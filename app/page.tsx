"use client";

import { useState } from "react";
import DownloadItem from "../components/download-item";
import DownloadFormModal from "../components/download-form-modal";
import { useDownloads } from "../components/download-context";
import { Card, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { DownloadIcon, Plus } from "lucide-react";
import DownloadBar from "@/components/download-bar";

export default function Page() {
  const { filteredDownloads, selectedMimeType, isLoading } = useDownloads();
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [url, setUrl] = useState("");

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
      <div className="mb-8">
        <DownloadBar
          setUrl={setUrl}
          url={url}
          filteredDownloads={filteredDownloads}
          selectedMimeType={selectedMimeType}
          setIsModalOpen={setIsModalOpen}
        />
      </div>

      <div className="space-y-4 px-3">
        <div className="flex items-center justify-between">
          <h2 className="text-xl font-semibold">
            Downloads ({filteredDownloads.length})
            {selectedMimeType && (
              <span className="text-sm font-normal text-muted-foreground ml-2">
                • Filtered
              </span>
            )}
          </h2>
        </div>

        {filteredDownloads.length === 0 ? (
          <Card>
            <CardContent className="flex flex-col items-center justify-center py-12">
              <DownloadIcon className="w-12 h-12 text-muted-foreground mb-4" />
              <h3 className="text-lg font-medium mb-2">
                {selectedMimeType
                  ? "No downloads match this filter"
                  : "No downloads yet"}
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
            {filteredDownloads.map((item) => (
              <DownloadItem download={item} key={item.id} />
            ))}
          </div>
        )}
      </div>

      <DownloadFormModal
        setUrl={setUrl}
        url={url}
        open={isModalOpen}
        onOpenChange={setIsModalOpen}
      />
    </div>
  );
}
