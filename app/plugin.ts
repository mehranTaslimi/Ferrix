import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

interface Context {
  id: string;
}

interface Ferrix {
  on: (event: string, cb: (ctx: Context) => Promise<void>) => () => Promise<void>;
}

declare global {
  interface Window {
    ferrix: Ferrix;
  }
}

(() => {
  if (typeof window === 'undefined') return;

  class Ferrix implements Ferrix {
    on = (event: string, cb: (ctx: Context) => Promise<void>) => {
      const id = crypto.randomUUID();

      invoke('register_event', { event, id });

      const fn = async (payload: Context) => {
        try {
          await cb(payload);
        } finally {
          await invoke('event_job_completed', { id, actionKind: payload });
        }
      };

      const unListen = listen<Context>(id, (ev) => fn(ev.payload));

      const unregister = async () => {
        invoke('unregister_event', { event, id });
        unListen.then((f) => f());
      };

      return unregister;
    };
  }

  window.ferrix = new Ferrix();
})();
