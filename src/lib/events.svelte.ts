import { listen, emit, type UnlistenFn } from "@tauri-apps/api/event";
import {
  library,
  type Current,
  type Track,
  type Duration,
} from "./state/state.svelte";
import { event } from "@tauri-apps/api";

type Unlisteners = {
  select: UnlistenFn;
  next: UnlistenFn;
  back: UnlistenFn;
  pause: UnlistenFn;
  play: UnlistenFn;
  stop: UnlistenFn;
  position: UnlistenFn;
  seek: UnlistenFn;
  cover: UnlistenFn;
};
type Listeners = {
  select: () => Promise<void>;
  next: () => Promise<void>;
  back: () => Promise<void>;
  pause: () => Promise<void>;
  play: () => Promise<void>;
  stop: () => Promise<void>;
  position: () => Promise<void>;
  seek: () => Promise<void>;
  cover: () => Promise<void>;
};

export const unlisteners = $state<Unlisteners>({
  select: () => {},
  next: () => {},
  back: () => {},
  pause: () => {},
  play: () => {},
  stop: () => {},
  position: () => {},
  seek: () => {},
  cover: () => {},
});
export const listeners = $state<Listeners>({
  select: async () => {
    unlisteners.select = await listen<Current>("select", (event) => {
      library.current = event.payload;
    });
    //INFO: check if other windows can pick up on this event
    await emit("songs-playing", library.current);
  },
  next: async () => {
    unlisteners.next = await listen<Current>("next", (event) => {
      library.current = event.payload;
    });
  },
  back: async () => {
    unlisteners.back = await listen<Current>("back", (event) => {
      library.current = event.payload;
    });
  },
  stop: async () => {
    // INFO: check whether playback will resume after calling stop, otherwise remove the return type
    unlisteners.stop = await listen<Current>("stop", (event) => {
      library.current = event.payload;
    });
  },
  play: async () => {
    unlisteners.play = await listen<boolean>("play", (event) => {
      if (library.current) {
        library.current.paused = event.payload;
      }
    });
  },
  pause: async () => {
    unlisteners.pause = await listen<boolean>("pause", (event) => {
      if (library.current) {
        library.current.paused = event.payload;
      }
    });
  },
  position: async () => {
    unlisteners.position = await listen<Duration>("position", (event) => {
      if (library.current) {
        library.position = event.payload;
      }
    });
  },
  seek: async () => {
    unlisteners.position = await listen<Duration>("seek", (event) => {
      library.position = event.payload;
    });
  },
  cover: async () => {
    unlisteners.cover = await listen<string>("cover", (event) => {
      if (event.payload) {
        library.cover = event.payload;
      }
    });
  },
});
