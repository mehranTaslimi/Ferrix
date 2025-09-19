import { invoke } from '@tauri-apps/api/core';

export default class Http {
  head = async (url: string) => {
    return await invoke('api_http_head', { url });
  };
}
