<script lang="ts">
import { invoke, Channel  } from "@tauri-apps/api/core";
import {library, type Track, type Current, type State, type Library} from "$lib/state/state.svelte"

let loading = $state(true);
$effect.pre(() => {
        const dispatcher = new Channel<string>();
        dispatcher.onmessage = (response) => {
        let songs = JSON.parse(response);
            library.songs = songs;
            loading = false;
    }
    invoke("songs", {dispatcher}).then(res  => console.log(res)).catch(err => console.log(err))
})

 $effect(() => {
    const dispatcher = new Channel<string>();
    dispatcher.onmessage = (message) => {
        let response: State = JSON.parse(message);
        console.log("state: ", response)
            library.current = response.current;
            library.next = response.next;
            library.prev = response.prev;
    }
    invoke("state", {dispatcher}).then(res => console.log(res)).catch(err => console.log(err));
})

async function select(id: number) {
        const dispatcher = new Channel<string>();
        dispatcher.onmessage = (response) => {
        let song: Current = JSON.parse(response);
        library.current = song;
    }
    invoke("select", {id: id, dispatcher: dispatcher}).then(res  => console.log(res)).catch(err => console.log(err))
    }


</script>
<main id="container">
    {#if loading}
        oading songs .....
    {:else}
    <section>
    {#each library.songs as [id, song]}
        <article >
            <div>
                <span>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M499.1 6.3c8.1 6 12.9 15.6 12.9 25.7v336c0 44.2-43 80-96 80s-96-35.8-96-80 43-80 96-80c11.2 0 22 1.6 32 4.6V147l-256 76.8V432c0 44.2-43 80-96 80S0 476.2 0 432s43-80 96-80c11.2 0 22 1.6 32 4.6V128c0-14.1 9.3-26.6 22.8-30.7l320-96c9.7-2.9 20.2-1.1 28.3 5z"/></svg>
                </span>
                <button onclick={() => select(id)}>
                    {song.title}
                </button>
            </div>
            <!-- svelte-ignore a11y_consider_explicit_label -->
            <button id="actions">
<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path d="M12 3c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2Zm0 14c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2Zm0-7c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2Z"/></svg>
            </button>
        </article>
    {:else}
        <p>No songs</p>
    {/each}
</section>
    {/if}
</main>


<style>
article {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    color: var(--text-color);
    padding: 10px;
    cursor: pointer;
    border-radius: var(--border-radius, 15px);
    background:rgba(255 255 255 /0.1);
    box-shadow: 0 8px 32px 0 rgb(31 38 135 / 0.37);
    backdrop-filter: blur(var(--blur));
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


article  #actions {
    width: 20px;
    left: 5px;
    border: none;
    cursor: pointer;
    margin-left: 15px;
    color: transparent;
    background-color: transparent;
}

article #actions svg {
    fill: var(--text-color);
}

section {
    display: flex;
    flex-direction: column;
    gap: 5px;
}

#container {
    display: flex;
    flex-direction: column;
}
</style>
