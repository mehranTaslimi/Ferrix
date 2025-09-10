import { listen } from '@tauri-apps/api/event';

import type { UnlistenFn } from '@tauri-apps/api/event';

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type EventCallbackFunction = (event: Event, payload: any) => Promise<void>;

export type RegistryAction = {
  event: string;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  payload: any;
};

export type EventKey = {
  event: string;
  id: string;
};

export type EventPayload = { action: RegistryAction; key: EventKey };

export default class Event {
  #parentEvent: Event | undefined;
  #registeredEvents: Array<Event> = [];
  #unListen: Promise<UnlistenFn> | undefined;
  #id: string = crypto.randomUUID();
  #event: string;
  #cb: EventCallbackFunction;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  #actionPayload: any;
  #isMuted = false;

  constructor(ev: string, cb: EventCallbackFunction, parent?: Event) {
    this.#parentEvent = parent;
    this.#event = ev;
    this.#cb = cb;

    this.#registerEvent();
  }

  #registerEvent = () => {
    window.ferrix.dispatch('register-event', { event_name: this.#event, event_id: this.#id });

    const fn = async (eventPayload: EventPayload) => {
      const { key, action } = eventPayload;
      const { payload } = action;

      this.#actionPayload = payload;

      try {
        await this.#cb(this, this.#actionPayload);
      } finally {
        this.#registeredEvents.forEach((r) => r.off());
        window.ferrix.dispatch('event-job-completed', {
          action_key: key,
          event_id: this.#id,
          muted_action: this.#isMuted ? { event: this.#event, payload: this.#actionPayload } : null,
        });
      }
    };

    const unListen = listen<EventPayload>(this.#id, ({ payload }) => {
      fn(payload);
    });

    this.#unListen = unListen;
  };

  on = (eventName: string, cb: EventCallbackFunction) => {
    const ev = new Event(eventName, cb, this);
    this.#registeredEvents.push(ev);

    return ev.off;
  };

  off = () => {
    this.#unListen?.then((f) => f());
    window.ferrix.dispatch('un-register-event', { event_name: this.#event, event_id: this.#id });
    this.#registeredEvents.forEach((r) => r.off());
  };

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  mutate = (p: any) => {
    this.#isMuted = true;
    this.#actionPayload = p;
  };
}
