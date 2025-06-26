import { memo } from "react";
import { Pause, Play } from "lucide-react";
import { Button } from "@/components/ui/button";
import { invoke } from "@tauri-apps/api/core";
import StatusAndSpeed from "./StatusAndSpeed";
import { Download, Status } from "./types";

function Item({ download }: { download: Download }) {

    const handleToggleDownload = async () => {
        if (download.status === Status.Paused) {
            await invoke("resume_download", { id: download.id })
        } else if (download.status === Status.Downloading) {
            await invoke("pause_download", { id: download.id })
        }
    }

    return (
        <li className="bg-neutral-300 shadow border border-neutral-100 rounded-sm p-2 w-full">
            <div className="flex justify-between items-center">
                <p className="max-w-1/3 overflow-hidden overflow-ellipsis whitespace-nowrap text-sm font-bold">
                    {download.file_name}
                </p>
                <div className="flex items-center gap-1">
                    {
                        (download.status !== Status.Completed && download.status !== Status.Failed) && <Button onClick={handleToggleDownload} className="mx-5" variant="outline">
                            {(download.status === Status.Paused || download.status === Status.Queued) ? <Play /> : <Pause />}
                        </Button>
                    }
                    <StatusAndSpeed id={download.id} totalBytes={download.total_bytes} downloadedBytes={download.downloaded_bytes} status={download.status} />
                </div>
            </div>
        </li>
    )
}

export default memo(Item);