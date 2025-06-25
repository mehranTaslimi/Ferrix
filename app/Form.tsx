import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

export default function Form() {
    const [value, setValue] = useState("https://caspian19.asset.aparat.com/aparat-video/9dcfe3d6b9a30b4b3706abb31cc2151d62651986-360p.mp4?wmsAuthSign=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ0b2tlbiI6ImQ0NzVhZGFiNjM3ZTIwOTVkMmY1ZjYzMzY2ODU4ZjBmIiwiZXhwIjoxNzUwNTQ4NDMzLCJpc3MiOiJTYWJhIElkZWEgR1NJRyJ9.GOxhbEA-eYErdeJQB6vEJGkbeS2K1W0REaebE-daEwQ");

    const handleSubmit = async () => {
        await invoke("add_download_queue", {
            url: value,
            chunk: 5
        });
    }

    return (
        <div className="mb-20 flex gap-2">
            <Input value={value} onChange={(ev) => setValue(ev.target.value)} />
            <Button variant="outline" onClick={handleSubmit}>Submit</Button>
        </div>
    )
}