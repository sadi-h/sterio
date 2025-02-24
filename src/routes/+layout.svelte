<script lang="ts">
import "../app.css";
let {children} = $props();
</script>

<main>

    <div>
        {@render children()}
    </div>
</main>

<style>
main {
    padding: 10px;
    width: 100vw;
    height: 100vh;
    overflow-x: hidden;
    background-color: #264F73;
}
div {
    border-radius: var(--border-radius, 10px);
    border: none;
    outline: 1px solid white;
    outline-offset: 5px;

}
</style>
