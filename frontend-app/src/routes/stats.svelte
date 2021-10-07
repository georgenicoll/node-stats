<svelte:head>
    <title>Stats</title>
</svelte:head>

<script context="module">

    const url = `/statistics.json`;
    const empty_statistics = { nodes: [] }

    export async function load({ page, fetch, session, stuff}) {
        const res = await fetch(url);
        if (res.ok) {
            return {
                props: {
                    statistics: await res.json(),
                    failure: ""
                }
            };
        }

        return {
            statistics: empty_statistics,
            failure: failureString(res)
        }
    }

    function failureString(res) {
        return `Failed to load ${url} with ${ res.statusText }`;
    }
</script>

<script>
    import NodeStats from '$lib/NodeStats.svelte';
    import { invalidate } from '$app/navigation';

    export let statistics;
    export let failure = "";

    async function refresh() {
        const res = await fetch(url);
        if (res.ok) {
            failure = "";
            statistics = await res.json();
        } else {
            statistics = empty_statistics;
            failure = failureString(res)
        }
    }
</script>

<style>
    .failure {
        color: red;
        font-weight: bolder;
    }
</style>

<h1>Node Statistics</h1>
{#each statistics.nodes as {name, stats}}
    <NodeStats nodeName={ name } nodeStats={ stats } />
{/each}

{#if failure.length > 0}
<p>
    <span class="failure">{ failure }</span>
</p>
{/if}

<button on:click={ refresh }>Refresh</button>