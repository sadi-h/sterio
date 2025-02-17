<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    let playing = $state(true);
</script>
<main>
        <img src="/img.webp" alt="standin-img">
    <div>

    <section id="content">
        <article>
                <div>
                    <span>
                        <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path d="M12 3c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2Zm0 14c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2Zm0-7c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2Z"/></svg>
                    </span>
                    <h1>  Ed Sheeran</h1>
                </div>
            <p>
                    <marquee behavior="scroll" direction="left">
                    Ed Sheeran  When Will I Be Alright Official Lyric Video
                    </marquee>
            </p>

        </article>
    </section>
    <section id="controls">
            <nav>
                <ul>
                <li>
<svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M112 64a16 16 0 0 1 16 16v136.43L360.77 77.11a35.13 35.13 0 0 1 35.77-.44c12 6.8 19.46 20 19.46 34.33v290c0 14.37-7.46 27.53-19.46 34.33a35.14 35.14 0 0 1-35.77-.45L128 295.57V432a16 16 0 0 1-32 0V80a16 16 0 0 1 16-16Z"/></svg>                </li>
                <li>
                   {#if playing}
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512"><path d="M48 64C21.5 64 0 85.5 0 112v288c0 26.5 21.5 48 48 48h32c26.5 0 48-21.5 48-48V112c0-26.5-21.5-48-48-48H48zm192 0c-26.5 0-48 21.5-48 48v288c0 26.5 21.5 48 48 48h32c26.5 0 48-21.5 48-48V112c0-26.5-21.5-48-48-48h-32z"/></svg>
                        {:else}
                    <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M133 440a35.37 35.37 0 0 1-17.5-4.67c-12-6.8-19.46-20-19.46-34.33V111c0-14.37 7.46-27.53 19.46-34.33a35.13 35.13 0 0 1 35.77.45l247.85 148.36a36 36 0 0 1 0 61l-247.89 148.4A35.5 35.5 0 0 1 133 440Z"/></svg>
                   {/if}
                </li>
                <li><svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M400 64a16 16 0 0 0-16 16v136.43L151.23 77.11a35.13 35.13 0 0 0-35.77-.44C103.46 83.47 96 96.63 96 111v290c0 14.37 7.46 27.53 19.46 34.33a35.14 35.14 0 0 0 35.77-.45L384 295.57V432a16 16 0 0 0 32 0V80a16 16 0 0 0-16-16Z"/></svg></li>
               </ul>
            <div id="outer">
                <div id="inner"></div>
            </div>
            </nav>
    </section>
    </div>
</main>

<style>
    main {
        --border-radius: 15px;
        --gap: 5px;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: var(--gap, 5px);
        max-height: 150px;
    }
    img {
        width: 150px;
        border: none;
        border-radius: var(--border-radius, 10px);
        object-fit: cover;
    }

    main section#content {
        padding-inline: 10px;
        padding-block: 2px;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 20px;
        border-radius: var(--border-radius, 10px);
        border: none;
    }

    main section#content article {
        display: flex;
        flex-direction: column;
        gap: 10px;
    }

    main section#content article h1 {
        color: rgb(255 255 255 / 0.8);
        max-height: 45px;
        overflow-y: hidden;
        overflow-wrap: break-word;
        position: relative;
    }
    main section#content article::after {
        content: '';
        position: absolute ;
        width: 40%;
        height: 0.5px;
        border-radius: var(--border-radius, 10px);
        left: 45%;
        bottom: 61%;
        background-color:  #42CDE7;
        opacity: 0.7;
    }

main section#content article p{
        font-size: 16px;
        max-height: 20px;
        color: #eff4f5;
        opacity: 0.8;
        overflow-y: hidden;
        overflow-wrap: break-word;
    }

main section#content article div{
        /* background-color: red; */
        display: flex;
        position: relative;
        align-items: flex-end;
        max-width: 190px;
        height: 50px;
    }

main section#content article div span{
        position: absolute;
        right: -22px;
        top: 0;
        fill: #eff4f5;
        width: 30px;
    }
main #controls {
        width: 100%;
        /* padding: 5px; */
        display: flex;
        align-items: center;
        justify-content: center;

    }
main #controls nav {
        display: flex;
        flex-direction: column;
        gap: 10px;
    }
main #controls nav ul {
        list-style: none;
        display: flex;
        gap: 50px;
        justify-content: center;
        align-items: center;
    }

main #controls nav ul li {
        width: 20px;
        /* height: 25px; */
    }

main #controls nav ul li svg{
        width: 100%;
        height: 100%;
        fill: #eff4f5;

    }

main #controls nav ul li svg:hover{
        fill: #42CDE7;

    }
main #controls nav ul li svg:active{
        fill: #ABF1BE;

    }
main #controls nav #outer{
        background-color: #eff4f5;
        height: 8px;
        border-radius: var(--border-radius, 10px);
    }
main #controls nav #inner{
        width: 50%;
        height: 100%;
        border-radius: var(--border-radius, 10px);
        background-color: #42CDE7;

    }
</style>
