import { invoke } from '@tauri-apps/api/core';

import Event from './event';

import type { EventCallbackFunction } from './event';
import type { InvokeArgs } from '@tauri-apps/api/core';

class Ferrix {
  #registeredEvents: Array<Event> = [];

  on = (eventName: string, cb: EventCallbackFunction): VoidFunction => {
    const ev = new Event(eventName, cb);
    this.#registeredEvents.push(ev);

    return ev.off;
  };

  dispatch = async (event: string, payload: InvokeArgs) => {
    await invoke('dispatch', { action: { event, payload } });
  };
}

export default Ferrix;
