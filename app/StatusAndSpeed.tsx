import { memo, useEffect, useMemo, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import { Status } from './types';

interface SpeedAndRemaining {
    speed: number,
    remaining_time: number,
    diskSpeed: number
}

function StatusAndSpeed({ id, totalBytes, status, downloadedBytes: currentDownloadedBytes }: { id: number, totalBytes: number, status: Status, downloadedBytes: number; }) {
    const [downloadedBytes, setDownloadedBytes] = useState(currentDownloadedBytes);
    const [speedAndRemaining, setSpeedAndRemaining] = useState<SpeedAndRemaining>({
        speed: 0,
        diskSpeed: 0,
        remaining_time: 0
    });


    useEffect(() => {
        const unListen1 = listen<number>(`downloaded_bytes_${id}`, (ev) => {
            setDownloadedBytes(ev.payload)
        });

        const unListen2 = listen<SpeedAndRemaining>(`speed_and_remaining_${id}`, (ev) => {
            setSpeedAndRemaining(prev => ({
                ...prev,
                ...ev.payload
            }))
        });

        const unListen3 = listen<number>(`disk_speed_${id}`, (ev) => {
            setSpeedAndRemaining(prev => ({ ...prev, diskSpeed: ev.payload }))
        });

        return () => {
            unListen1.then(fn => fn())
            unListen2.then(fn => fn())
            unListen3.then(fn => fn())
        }
    }, [id]);


    const remainingTime = useMemo(() => {
        const second = Math.round(speedAndRemaining.remaining_time);
        const minute = Math.round(speedAndRemaining.remaining_time / 60);
        const hour = Math.round(speedAndRemaining.remaining_time / 60 / 60);
        const day = Math.round(speedAndRemaining.remaining_time / 60 / 60 / 24);

        if (day >= 1) {
            return day > 1 ? day + " days" : day + " day"
        }
        if (hour >= 1) {
            return hour > 1 ? hour + " hours" : hour + " hour"
        }
        if (minute >= 1) {
            return minute > 1 ? minute + " minutes" : minute + " minute"
        }

        return second > 1 ? second + " seconds" : second + " second"

    }, [speedAndRemaining.remaining_time])

    const speed = useMemo(() => {
        const kb = Math.round(speedAndRemaining.speed);
        const mb = Math.round(speedAndRemaining.speed / 1024);
        const gb = Math.round(speedAndRemaining.speed / 1024 / 1024);

        if (gb >= 1) {
            return gb + " GB/s"
        }
        if (mb >= 1) {
            return mb + " MB/s"
        }

        return kb + " KB/s"

    }, [speedAndRemaining.speed])

    const diskSpeed = useMemo(() => {
        const kb = Math.round(speedAndRemaining.diskSpeed);
        const mb = Math.round(speedAndRemaining.diskSpeed / 1024);
        const gb = Math.round(speedAndRemaining.diskSpeed / 1024 / 1024);

        if (gb >= 1) {
            return gb + " GB/s"
        }
        if (mb >= 1) {
            return mb + " MB/s"
        }

        return kb + " KB/s"

    }, [speedAndRemaining.diskSpeed])

    const progress = useMemo(() => {
        return Math.round((downloadedBytes / totalBytes) * 100)
    }, [downloadedBytes, totalBytes])


    return (
        <>
            {status === Status.Downloading && <p className="text-white text-xs bg-neutral-600 p-1 mx-1 rounded-xs border border-white">{remainingTime} left</p>}
            {status === Status.Downloading && <p className="text-white text-xs bg-neutral-600 p-1 mx-1 rounded-xs border border-white">Net: {speed}</p>}
            {status === Status.Downloading && <p className="text-white text-xs bg-neutral-600 p-1 mx-1 rounded-xs border border-white">Disk: {diskSpeed}</p>}
            {status === Status.Downloading && <p className="text-white text-xs bg-neutral-600 p-1 mx-1 rounded-xs border border-white">{progress}%</p>}
            <p className="text-white text-xs bg-neutral-600 p-1 mx-1 rounded-xs border border-white uppercase">{status}</p>
        </>
    )
}

export default memo(StatusAndSpeed)