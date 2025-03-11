<script lang="ts">
  import { listen, emit, type UnlistenFn } from "@tauri-apps/api/event";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { invoke, Channel } from "@tauri-apps/api/core";
  import { unlisteners, listeners } from "$lib/events.svelte";
  import {
    library,
    type Track,
    type Current,
    type State,
    type Library,
  } from "$lib/state/state.svelte";

  $effect(() => {
    const dispatcher = new Channel<string>();
    dispatcher.onmessage = (message) => {
      let response: State = JSON.parse(message);
      library.current = response.current;
      library.next = response.next;
      library.prev = response.prev;
    };
    invoke("state", { dispatcher })
      .then((res) => console.log(res))
      .catch((err) => console.log(err));

    //set up event listeners
    listeners.select();
    listeners.play();
    listeners.pause();
    listeners.stop();
    return () => {
      //remove event listeners
      unlisteners.select();
      unlisteners.play();
      unlisteners.pause();
    };
  });

  async function newWindow(e: Event) {
    e.preventDefault();
    invoke("video_window")
      .then((res) => console.log(res))
      .catch((err) => console.log(err));
  }

  async function pause(event: Event) {
    event.preventDefault();
    const dispatcher = new Channel<string>();
    dispatcher.onmessage = (response) => {
      //response is empty because nothing gets dispatched back on channel
    };
    invoke("pause", { dispatcher })
      .then((res) => console.log(res))
      .catch((err) => console.log(err));
  }

  async function play(event: Event) {
    event.preventDefault();
    const dispatcher = new Channel<string>();
    dispatcher.onmessage = (response) => {
      //response is empty because nothing gets dispatched back on channel
    };
    invoke("play", { dispatcher })
      .then((res) => console.log(res))
      .catch((err) => console.log(err));
  }

  let selected: Current = $state({
    song: {
      artist: "",
      title: "",
      len: { secs: 0, nanos: 0 },
      album: {
        title: "",
        artist: "",
        cover: {
          mime_type: "",
          data: "",
        },
        tracks: 0,
        year: 0,
      },
      track: 0,
      genre: "",
      source: "",
    },
    paused: false,
  });

  // TODO: spin/markup for loading state
  let loading = $state(true);
  let showSongActions = $state(false);

  $effect(() => {
    const dispatcher = new Channel<string>();
    dispatcher.onmessage = (response) => {
      let songs = JSON.parse(response);
      console.log(songs);
      library.songs = songs;
      loading = false;
    };
    invoke("songs", { dispatcher })
      .then((res) => console.log(res))
      .catch((err) => console.log(err));
  });

  async function select(id: number) {
    const dispatcher = new Channel<string>();
    dispatcher.onmessage = (response) => {
      let song: Current = JSON.parse(response);
      library.current = song;
    };
    invoke("select", { id: id, dispatcher: dispatcher })
      .then((res) => console.log(res))
      .catch((err) => console.log(err));
  }

  $inspect(library.current?.paused);
</script>

<main id="container">
  {#if loading}
    Fetching Songs
  {:else}
    <section>
      {#each library.songs as [id, song] (id)}
        <!-- content here -->
        <article class:current={library.current?.song.source === song.source}>
          <div>
            <span>
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"
                ><path
                  d="M499.1 6.3c8.1 6 12.9 15.6 12.9 25.7v336c0 44.2-43 80-96 80s-96-35.8-96-80 43-80 96-80c11.2 0 22 1.6 32 4.6V147l-256 76.8V432c0 44.2-43 80-96 80S0 476.2 0 432s43-80 96-80c11.2 0 22 1.6 32 4.6V128c0-14.1 9.3-26.6 22.8-30.7l320-96c9.7-2.9 20.2-1.1 28.3 5z"
                /></svg
              >
            </span>
            <button onclick={() => select(id)}>
              {song.title}
            </button>
          </div>
          <section id="action_buttons">
            {#if library.current?.song.source === song.source}
              <!-- content here -->
              {#if library.current?.paused}
                <!-- svelte-ignore a11y_consider_explicit_label -->
                <button
                  id="play"
                  onclick={(e) => {
                    if (library.current) {
                      library.current.paused = false;
                    }
                    play(e);
                  }}
                >
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="512"
                    height="512"
                    viewBox="0 0 512 512"
                    ><path
                      d="M133 440a35.37 35.37 0 0 1-17.5-4.67c-12-6.8-19.46-20-19.46-34.33V111c0-14.37 7.46-27.53 19.46-34.33a35.13 35.13 0 0 1 35.77.45l247.85 148.36a36 36 0 0 1 0 61l-247.89 148.4A35.5 35.5 0 0 1 133 440Z"
                    /></svg
                  >
                </button>
              {:else}
                <!-- else content here -->
                <!-- svelte-ignore a11y_consider_explicit_label -->
                <button
                  id="pause"
                  onclick={(e) => {
                    if (library.current) {
                      library.current.paused = true;
                    }
                    pause(e);
                  }}
                >
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512"
                    ><path
                      d="M48 64C21.5 64 0 85.5 0 112v288c0 26.5 21.5 48 48 48h32c26.5 0 48-21.5 48-48V112c0-26.5-21.5-48-48-48H48zm192 0c-26.5 0-48 21.5-48 48v288c0 26.5 21.5 48 48 48h32c26.5 0 48-21.5 48-48V112c0-26.5-21.5-48-48-48h-32z"
                    /></svg
                  >
                </button>
              {/if}
            {/if}

            <!-- svelte-ignore a11y_consider_explicit_label -->
            <button
              id="actions"
              onclick={(e) => {
                e.preventDefault();
                showSongActions = true;
              }}
            >
              <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"
                ><path
                  d="M12 3c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2Zm0 14c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2Zm0-7c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2Z"
                /></svg
              >
            </button>
          </section>
        </article>
      {:else}
        <p>
          No songs. Please browse your device and choose folders with mp3 songs
          to play
        </p>
      {/each}
    </section>
  {/if}

  <nav
    id="song_actions"
    class={showSongActions ? "show_actions" : "hide_actions"}
  >
    <button
      aria-labelledby="close actions"
      onclick={() => {
        showSongActions = false;
      }}
      ><svg
        xmlns="http://www.w3.org/2000/svg"
        width="512"
        height="512"
        viewBox="0 0 512 512"
        ><path
          d="M256 48C141.31 48 48 141.31 48 256s93.31 208 208 208 208-93.31 208-208S370.69 48 256 48Zm75.31 260.69a16 16 0 1 1-22.62 22.62L256 278.63l-52.69 52.68a16 16 0 0 1-22.62-22.62L233.37 256l-52.68-52.69a16 16 0 0 1 22.62-22.62L256 233.37l52.69-52.68a16 16 0 0 1 22.62 22.62L278.63 256Z"
        /></svg
      ></button
    >
    <ul>
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
      <li onclick={newWindow}>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          fill="currentColor"
          class="bi bi-collection-play-fill"
          viewBox="0 0 16 16"
          ><path
            d="M2.5 3.5a.5.5 0 0 1 0-1h11a.5.5 0 0 1 0 1h-11zm2-2a.5.5 0 0 1 0-1h7a.5.5 0 0 1 0 1h-7zM0 13a1.5 1.5 0 0 0 1.5 1.5h13A1.5 1.5 0 0 0 16 13V6a1.5 1.5 0 0 0-1.5-1.5h-13A1.5 1.5 0 0 0 0 6v7zm6.258-6.437a.5.5 0 0 1 .507.013l4 2.5a.5.5 0 0 1 0 .848l-4 2.5A.5.5 0 0 1 6 12V7a.5.5 0 0 1 .258-.437z"
          /></svg
        >
        <span>Play all</span>
      </li>

      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
      <li onclick={newWindow}>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="512"
          height="512"
          viewBox="0 0 512 512"
          ><path
            d="M133 440a35.37 35.37 0 0 1-17.5-4.67c-12-6.8-19.46-20-19.46-34.33V111c0-14.37 7.46-27.53 19.46-34.33a35.13 35.13 0 0 1 35.77.45l247.85 148.36a36 36 0 0 1 0 61l-247.89 148.4A35.5 35.5 0 0 1 133 440Z"
          /></svg
        >
        <span class="secondary">
          <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"
            ><path d="M11 11V5h2v6h6v2h-6v6h-2v-6H5v-2h6Z" /></svg
          >
        </span>
        <span>Append</span>
      </li>
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
      <li onclick={newWindow}>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="512"
          height="512"
          viewBox="0 0 512 512"
          ><path
            d="M133 440a35.37 35.37 0 0 1-17.5-4.67c-12-6.8-19.46-20-19.46-34.33V111c0-14.37 7.46-27.53 19.46-34.33a35.13 35.13 0 0 1 35.77.45l247.85 148.36a36 36 0 0 1 0 61l-247.89 148.4A35.5 35.5 0 0 1 133 440Z"
          /></svg
        >
        <span class="secondary">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="512"
            height="512"
            viewBox="0 0 512 512"
            ><path
              d="M400 64a16 16 0 0 0-16 16v136.43L151.23 77.11a35.13 35.13 0 0 0-35.77-.44C103.46 83.47 96 96.63 96 111v290c0 14.37 7.46 27.53 19.46 34.33a35.14 35.14 0 0 0 35.77-.45L384 295.57V432a16 16 0 0 0 32 0V80a16 16 0 0 0-16-16Z"
            /></svg
          >
        </span>

        <span>Insert next</span>
      </li>

      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
      <li onclick={newWindow}>
        <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"
          ><path
            d="M12 22C6.477 22 2 17.523 2 12S6.477 2 12 2s10 4.477 10 10-4.477 10-10 10Zm-1-11v6h2v-6h-2Zm0-4v2h2V7h-2Z"
          /></svg
        >
        <span>Information</span>
      </li>

      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
      <li onclick={newWindow}>
        <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"
          ><path
            d="M2 18h10v2H2v-2Zm0-7h20v2H2v-2Zm0-7h20v2H2V4Zm16 14v-3h2v3h3v2h-3v3h-2v-3h-3v-2h3Z"
          /></svg
        > <span>Add to playlist</span>
      </li>

      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
      <li onclick={newWindow}>
        <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"
          ><path
            d="M17 6h5v2h-2v13a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V8H2V6h5V3a1 1 0 0 1 1-1h8a1 1 0 0 1 1 1v3Zm-8 5v6h2v-6H9Zm4 0v6h2v-6h-2ZM9 4v2h6V4H9Z"
          /></svg
        > <span>Remove</span>
      </li>

      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
      <li onclick={newWindow}>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="512"
          height="512"
          viewBox="0 0 512 512"
          ><path
            d="M384 336a63.78 63.78 0 0 0-46.12 19.7l-148-83.27a63.85 63.85 0 0 0 0-32.86l148-83.27a63.8 63.8 0 1 0-15.73-27.87l-148 83.27a64 64 0 1 0 0 88.6l148 83.27A64 64 0 1 0 384 336Z"
          /></svg
        > <span>Share</span>
      </li>
    </ul>
  </nav>
</main>

<style>
  #song_actions {
    border-radius: var(--border-radius, 15px);
    background: rgba(255 255 255 /0.1);
    box-shadow: 0 8px 32px 0 rgb(31 38 135 / 0.37);
    backdrop-filter: blur(var(--blur));
    -webkit-backdrop-filter: blur(var(--blur));
    position: fixed;
    margin-inline: auto;
    height: 55vh;
    left: 5px;
    bottom: 2px;
    margin-top: 50vh;
    width: 97.2%;
    padding-inline: 15px;
    padding-block: 20px;
  }

  #song_actions button {
    position: absolute;
    width: var(--svg-size, 25px);
    left: 5px;
    top: 5px;
    border: none;
    color: transparent;
    background-color: transparent;
    cursor: pointer;
  }

  #song_actions button svg {
    width: 100%;
    height: 100%;
    fill: var(--hover-color);
  }

  .show_actions {
    transition: transform 400ms linear;
    transform: translateY(0px);
  }
  .hide_actions {
    transition: transform 400ms linear;
    transform: translateY(400px);
  }

  .secondary {
    margin-left: -20px;
    margin-top: 5px;
    margin-right: 3px;
    height: 12px;
    width: 12px;
  }
  .secondary svg {
    height: 100%;
    width: 100% !important;
  }

  #song_actions ul {
    list-style-type: none;
    display: flex;
    gap: 2px;
    flex-direction: column;
    min-width: 150px;
    padding: 10px;
  }

  #song_actions ul li {
    display: flex;
    gap: 5px;
  }

  #song_actions ul li svg {
    width: 20px;
    height: 20px;
    margin-right: 10px;
    fill: var(--text-color);
  }

  #song_actions ul li {
    display: flex;
    align-items: center;
    justify-content: flex-start;
    padding: 5px;
    color: var(--text-color);
    cursor: pointer;
  }

  #song_actions ul li:hover,
  #song_actions ul li:hover svg {
    color: var(--hover-color);
    fill: var(--hover-color);
  }

  main {
    position: relative;
  }
  .current {
    background: rgba(255 255 255 /0.3);
    box-shadow: 0 8px 32px 0 rgb(31 38 135 / 0.37);
    backdrop-filter: blur(var(--blur));
  }

  article:hover {
    background: rgba(255 255 255 /0.3);
    box-shadow: 0 8px 32px 0 rgb(31 38 135 / 0.37);
    backdrop-filter: blur(var(--blur));
  }

  article {
    margin-block: 5px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 15px;
    color: var(--text-color);
    padding: 10px;
    cursor: pointer;
    border-radius: var(--border-radius, 15px);
    background: rgba(255 255 255 /0.1);
    box-shadow: 0 8px 32px 0 rgb(31 38 135 / 0.37);
    backdrop-filter: blur(var(--blur));
  }

  #action_buttons {
    min-width: 70px;
    display: flex;
    align-items: center;
    justify-content: end;
  }

  article div {
    display: flex;
    gap: 15px;
    align-items: center;
    overflow: hidden;
  }
  article div span {
    display: block;
    width: 15px;
  }

  article div span svg {
    margin-right: 10px;
    width: 100%;
    height: 100%;
    fill: var(--text-color);
  }

  article div button:nth-of-type(1) {
    background-color: transparent;
    border: none;
    color: var(--text-color);
    font-size: 12px;
    text-wrap-mode: nowrap;
  }

  #actions {
    width: 20px;
    left: 5px;
    border: none;
    cursor: pointer;
    margin-left: 15px;
    color: transparent;
    background-color: transparent;
  }

  #actions svg {
    fill: var(--text-color);
  }

  #pause {
    width: 20px;
    height: 20px;
    border: none;
    cursor: pointer;
    background-color: transparent;
  }

  #pause svg {
    width: 100%;
    height: 100%;
    color: white;
    fill: var(--text-color);
  }

  #play {
    width: 20px;
    height: 20px;
    border: none;
    cursor: pointer;
    background-color: transparent;
  }

  #play svg {
    width: 100%;
    height: 100%;
    color: white;
    fill: var(--text-color);
  }

  section::not(#action_buttons) {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }

  #container {
    display: flex;
    flex-direction: column;
  }
</style>
