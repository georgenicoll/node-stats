<svelte:head>
    <title>Stats</title>
</svelte:head>

<script>
    import NodeStats from '$lib/NodeStats.svelte';
    import { onMount } from 'svelte';
    import { writable } from 'svelte/store'

    const empty_statistics = { nodes: [] }
    const url = `/statistics.json`;

    const statistics = writable(empty_statistics);
    const failure = writable("");

    onMount(refresh);

    async function refresh() {
        const res = await fetch(url);
        if (res.ok) {
            failure.set("");
            await res.json().then(stats => statistics.set(stats))
        } else {
            statistics.set(empty_statistics);
            failure.set(failureString(res));
        }
    }

    function failureString(res) {
        return `Failed to load ${url} with ${ res.statusText }`;
    }
</script>

<style>
    .failure {
        color: red;
        font-weight: bolder;
    }
</style>

<div >
<h1>Node Statistics</h1>
{#each $statistics.nodes as {name, stats}}
    <NodeStats nodeName={ name } nodeStats={ stats } />
{/each}

{#if $failure.length > 0}
<p>
    <span class="failure">{ failure }</span>
</p>
{/if}
</div>

<button on:click={ refresh }>Refresh</button>