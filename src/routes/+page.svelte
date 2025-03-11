<script lang="ts">
  import { invoke, Channel } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import {
    library,
    type Track,
    type Current,
    type State,
    type Library,
    type Duration,
  } from "$lib/state/state.svelte";
  import { unlisteners, listeners } from "$lib/events.svelte";

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
    listeners.next();
    listeners.back();

    listeners.stop();
    listeners.position();
    listeners.seek();
    listeners.cover();
    return () => {
      //remove event listeners
      unlisteners.select();
      unlisteners.position();
      unlisteners.play();
      unlisteners.pause();
      unlisteners.next();
      unlisteners.back();
      unlisteners.stop();
      unlisteners.seek();
      unlisteners.cover();
    };
  });
  $effect(() => {});

  $effect(() => {
    let source = "";
    if (library.position) {
      if (library.current) {
        if (library.position.secs == library.current.song.len?.secs) {
          const dispatcher = new Channel<string>();
          dispatcher.onmessage = (response) => {};
          invoke("next", { dispatcher })
            .then((_res) => {})
            .catch((_err) => {});
        }
      }
    }
  });

  $effect(() => {
    let timer: number;
    if (library.current && !library.current.paused) {
      const dispatcher = new Channel<string>();
      dispatcher.onmessage = (response) => {
        // response is empty. nothing dispatched  back on channel
      };
      timer = setInterval(() => {
        //TODO: use res for notification or alike
        invoke("position", { dispatcher })
          .then((_res) => {})
          .catch((_err) => {});
      }, 1000);
    }
    return () => {
      clearInterval(timer);
    };
  });

  async function next(event: Event) {
    event.preventDefault();
    let source = "";
    const dispatcher = new Channel<string>();
    dispatcher.onmessage = (response) => {};
    invoke("next", { dispatcher })
      .then((res) => console.log(res))
      .catch((err) => console.log(err));
  }

  async function back(event: Event) {
    event.preventDefault();
    const dispatcher = new Channel<string>();
    dispatcher.onmessage = (response) => {};
    invoke("back", { dispatcher })
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

  async function load(event: Event) {
    event.preventDefault();
    const dispatcher = new Channel<string>();
    dispatcher.onmessage = (response) => {
      let songs = JSON.parse(response);
      library.songs = songs;
      console.log(songs);
    };
    try {
      const files = await open({
        multiple: true,
        directory: true,
      });
      let response = await invoke("load", { dirs: files, dispatcher });
    } catch (error) {
      console.log(error);
    }
  }
  async function seek(dur: number) {
    const dispatcher = new Channel<string>();
    dispatcher.onmessage = (response) => {
      let current: Duration = JSON.parse(response);
    };
    invoke("seek", { dur, dispatcher })
      .then((res) => console.log(res))
      .catch((err) => console.log(err));
  }

  async function select(id: number) {
    const dispatcher = new Channel<string>();
    dispatcher.onmessage = (response) => {};
    invoke("select", { id: id, dispatcher: dispatcher })
      .then((res) => console.log(res))
      .catch((err) => console.log(err));
  }

  async function musicWindow(e: Event) {
    e.preventDefault();
    invoke("music_window")
      .then((res) => console.log(res))
      .catch((err) => console.log(err));
  }
  async function VideoWindow(e: Event) {
    e.preventDefault();
    invoke("video_window")
      .then((res) => console.log(res))
      .catch((err) => console.log(err));
  }
  async function playListWindow(e: Event) {
    e.preventDefault();
    invoke("video_window")
      .then((res) => console.log(res))
      .catch((err) => console.log(err));
  }
</script>

<main>
  <img
    src={library.current && library.current.song.album.cover
      ? library.ui.base64ToFile(library.current.song.album.cover.data)
      : "/img3.jpg"}
    alt="standin-img"
  />
  <div>
    <section id="content">
      <article>
        <div>
          <!-- button to toggle actions -->
          <button
            aria-labelledby="actions button"
            onclick={() => {
              library.ui.showActions = true;
            }}
          >
            <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"
              ><path
                d="M12 3c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2Zm0 14c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2Zm0-7c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2Z"
              /></svg
            >
          </button>
          <h1 style="text-transform: uppercase;">
            {library.current?.song.artist ? library.current.song.artist : ""}
          </h1>
        </div>
        <p id="song_name">
          <!-- svelte-ignore a11y_distracting_elements -->
          <!--TODO: make custom marquee -->
          {#if library.current}
            <!-- content here -->
            <marquee behavior="scroll" direction="left">
              {library.current.song.title}
            </marquee>
          {/if}
        </p>
      </article>
    </section>
    {#if library.current}
      <!-- content here -->
      <section id="controls">
        <nav>
          <ul>
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
            <li onclick={back}>
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="512"
                height="512"
                viewBox="0 0 512 512"
                ><path
                  d="M112 64a16 16 0 0 1 16 16v136.43L360.77 77.11a35.13 35.13 0 0 1 35.77-.44c12 6.8 19.46 20 19.46 34.33v290c0 14.37-7.46 27.53-19.46 34.33a35.14 35.14 0 0 1-35.77-.45L128 295.57V432a16 16 0 0 1-32 0V80a16 16 0 0 1 16-16Z"
                /></svg
              >
            </li>
            <li>
              {#if !library.current || library.current?.paused}
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <svg
                  onclick={(e) => {
                    play(e);
                  }}
                  xmlns="http://www.w3.org/2000/svg"
                  width="512"
                  height="512"
                  viewBox="0 0 512 512"
                  ><path
                    d="M133 440a35.37 35.37 0 0 1-17.5-4.67c-12-6.8-19.46-20-19.46-34.33V111c0-14.37 7.46-27.53 19.46-34.33a35.13 35.13 0 0 1 35.77.45l247.85 148.36a36 36 0 0 1 0 61l-247.89 148.4A35.5 35.5 0 0 1 133 440Z"
                  /></svg
                >
              {:else}
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <svg
                  onclick={(e) => {
                    pause(e);
                  }}
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 320 512"
                  ><path
                    d="M48 64C21.5 64 0 85.5 0 112v288c0 26.5 21.5 48 48 48h32c26.5 0 48-21.5 48-48V112c0-26.5-21.5-48-48-48H48zm192 0c-26.5 0-48 21.5-48 48v288c0 26.5 21.5 48 48 48h32c26.5 0 48-21.5 48-48V112c0-26.5-21.5-48-48-48h-32z"
                  /></svg
                >
              {/if}
            </li>
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
            <li onclick={next}>
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="512"
                height="512"
                viewBox="0 0 512 512"
                ><path
                  d="M400 64a16 16 0 0 0-16 16v136.43L151.23 77.11a35.13 35.13 0 0 0-35.77-.44C103.46 83.47 96 96.63 96 111v290c0 14.37 7.46 27.53 19.46 34.33a35.14 35.14 0 0 0 35.77-.45L384 295.57V432a16 16 0 0 0 32 0V80a16 16 0 0 0-16-16Z"
                /></svg
              >
            </li>
          </ul>
          <div id="outer">
            <div
              id="inner"
              style="width:{library.ui.positionPercent(
                library.position.secs,
                library.current ? library.current.song.len.secs : 0,
              )}%"
            >
              {#if library.current}
                <input
                  type="range"
                  name="position"
                  id="position"
                  step="1"
                  bind:value={() => library.position.secs, (v) => seek(v)}
                  max={library.current.song.len.secs}
                />
              {/if}
            </div>
          </div>
          <div id="time">
            {#if library.current}
              <small
                >{library.current
                  ? library.ui.timePretty(library.current.song.len.secs)
                  : ""}</small
              >
              <small
                >{library.position
                  ? library.ui.timePretty(library.position.secs)
                  : ""}</small
              >
            {/if}
          </div>
        </nav>
      </section>
    {/if}
  </div>
  <nav
    id="actions"
    class={library.ui.showActions ? "show_actions" : "hide_actions"}
  >
    <button
      aria-labelledby="close actions"
      onclick={() => (library.ui.showActions = false)}
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
      <li onclick={musicWindow}>
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"
          ><path
            d="M499.1 6.3c8.1 6 12.9 15.6 12.9 25.7v336c0 44.2-43 80-96 80s-96-35.8-96-80 43-80 96-80c11.2 0 22 1.6 32 4.6V147l-256 76.8V432c0 44.2-43 80-96 80S0 476.2 0 432s43-80 96-80c11.2 0 22 1.6 32 4.6V128c0-14.1 9.3-26.6 22.8-30.7l320-96c9.7-2.9 20.2-1.1 28.3 5z"
          /></svg
        >
        <span>Audio</span>
      </li>
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
      <li onclick={VideoWindow}>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="512"
          height="512"
          viewBox="0 0 512 512"
          ><path
            d="M464 384.39a32 32 0 0 1-13-2.77 15.77 15.77 0 0 1-2.71-1.54l-82.71-58.22A32 32 0 0 1 352 295.7v-79.4a32 32 0 0 1 13.58-26.16l82.71-58.22a15.77 15.77 0 0 1 2.71-1.54 32 32 0 0 1 45 29.24v192.76a32 32 0 0 1-32 32ZM268 400H84a68.07 68.07 0 0 1-68-68V180a68.07 68.07 0 0 1 68-68h184.48A67.6 67.6 0 0 1 336 179.52V332a68.07 68.07 0 0 1-68 68Z"
          />
          <span>Video</span></svg
        >
      </li>
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
      <li onclick={playListWindow}>
        <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"
          ><path
            d="M2 18h10v2H2v-2Zm0-7h14v2H2v-2Zm0-7h20v2H2V4Zm17 11.17V9h5v2h-3v7a3 3 0 1 1-2-2.83Z"
          /></svg
        >
        <span>Playlist</span>
      </li>
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
      <li onclick={load}>
        <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"
          ><path
            d="M12.414 5H21a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h7.414l2 2ZM11 13.05a2.5 2.5 0 1 0 2 2.45V11h3V9h-5v4.05Z"
          /></svg
        >
        <span>Browse</span>
      </li>
    </ul>
  </nav>
</main>

<style>
  main {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--gap);
    max-height: 150px;
    position: relative;
  }

  #actions {
    display: flex;
    justify-content: flex-end;
    position: absolute;
    right: 0px;
    height: 152px;
    width: 100%;
    overflow: hidden;
    border-radius: var(--border-radius, 15px);
    background: rgba(255 255 255 /0.1);
    box-shadow: 0 8px 32px 0 rgb(31 38 135 / 0.37);
    backdrop-filter: blur(var(--blur));
    -webkit-backdrop-filter: blur(var(--blur));
  }

  #actions button {
    position: absolute;
    width: var(--svg-size, 25px);
    left: 5px;
    top: 5px;
    border: none;
    color: transparent;
    background-color: transparent;
    cursor: pointer;
  }

  #actions button svg {
    width: 100%;
    height: 100%;
    fill: var(--hover-color);
  }

  .show_actions {
    transition: transform 400ms linear;
    transform: translate(0px);
  }
  .hide_actions {
    transition: transform 400ms linear;
    transform: translate(400px);
  }

  #actions ul {
    list-style-type: none;
    display: flex;
    gap: 5px;
    flex-direction: column;
    min-width: 150px;
    padding: 10px;
  }

  #actions ul li svg {
    width: 20px;
    height: 20px;
    margin-right: 10px;
    fill: var(--text-color);
  }

  #actions ul li {
    display: flex;
    align-items: center;
    justify-content: flex-start;
    padding: 5px;
    color: var(--text-color);
    cursor: pointer;
  }

  #actions ul li:hover,
  #actions ul li:hover svg {
    color: var(--hover-color);
    fill: var(--hover-color);
  }

  main img {
    max-width: 150px;
    max-height: 150px;
    min-width: 150px;
    min-height: 150px;
    border: none;
    border-radius: var(--border-radius, 10px);
    object-fit: cover;
  }

  #content {
    padding-inline: 10px;
    padding-block: 2px;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 20px;
    border-radius: var(--border-radius, 10px);
    border: none;
  }

  #content article {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  #song_name {
    min-width: 195px;
    max-width: 195px;
    margin-block: 5px;
    color: var(--white-color);
    font-size: 18px;
  }

  #content article h1 {
    color: var(--white-color);
    max-height: 45px;
    overflow-y: hidden;
    overflow-wrap: break-word;
    position: relative;
  }

  #content article::after {
    content: "";
    position: absolute;
    width: 40%;
    height: 0.5px;
    border-radius: var(--border-radius, 10px);
    left: 45%;
    bottom: 61%;
    background-color: var(--hover-color);
    opacity: 0.5;
  }

  #content article p {
    font-size: 16px;
    max-height: 20px;
    color: var(--white-color);
    overflow-y: hidden;
    overflow-wrap: break-word;
  }

  #content article div {
    display: flex;
    position: relative;
    align-items: flex-end;
    max-width: 190px;
    height: 50px;
  }

  #content article div button {
    display: none;
    position: absolute;
    top: 5px;
    right: -17px;
    background-color: transparent;
    border: none;
    cursor: pointer;
  }

  #content:hover article div button {
    display: block;
  }

  #content article div button svg:hover {
    fill: var(--hover-color);
  }

  #content article div button svg:active {
    fill: var(--active-color);
  }

  #content article div button svg {
    width: 30px;
    fill: var(--text-color);
  }

  #controls {
    width: 100%;
    height: 60px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  #controls nav {
    display: flex;
    flex-direction: column;
    gap: 2px;
    margin-bottom: 5px;
  }

  #controls nav ul {
    height: 30px;
    list-style: none;
    display: flex;
    gap: 50px;
    justify-content: center;
    align-items: center;
  }

  #controls nav ul li {
    width: 20px;
  }

  #controls nav ul li svg {
    width: 100%;
    height: 100%;
    fill: var(--text-color);
  }

  #controls nav ul li svg:hover {
    fill: var(--hover-color);
  }

  #controls nav ul li svg:active {
    fill: var(--active-color);
  }

  #controls nav #outer {
    background-color: var(--text-color);
    height: 8px;
    max-width: 100%;
    border-radius: var(--border-radius, 10px);
    margin-top: 5px;
    position: relative;
  }

  #controls nav #inner {
    height: 100%;
    border-radius: var(--border-radius, 10px);
    background-color: var(--hover-color);
  }
  #controls nav #time {
    display: flex;
    justify-content: space-between;
    align-items: center;
    color: var(--text-color);
    opacity: 0.7;
  }

  #inner input[type="range"]::-webkit-slider-runnable-track {
    cursor: pointer;
    background-color: transparent;
    border-radius: var(--border-radius, 10px);
  }

  #inner input[type="range"]::-moz-range-track {
    background-color: transparent;
    border-radius: var(--border-radius, 10px);
  }

  #position {
    display: none;
    position: absolute;
    top: -4px;
    width: 100%;
  }

  #controls nav:hover #position {
    display: block;
  }
</style>
