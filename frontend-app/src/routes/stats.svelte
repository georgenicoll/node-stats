<svelte:head>
    <title>Stats</title>
</svelte:head>

<script>
    import NodeStats from '$lib/NodeStats.svelte';
    import { onMount } from 'svelte';
    import { writable } from 'svelte/store'
    import { Styles, Card, CardHeader, CardTitle, CardBody, Alert, Button } from 'sveltestrap';

    const empty_statistics = { nodes: [] }
    const url = `/statistics.json`;

    const statistics = writable(empty_statistics);
    const failure = writable("");

    onMount(refresh);

    async function refresh() {
        const res = await fetch(url);
        if (res.ok) {
            failure.set("");
            await res.json().then(stats => {
                stats.nodes.sort((a, b) => {
                    return a.key.localeCompare(b.key);
                });
                statistics.set(stats);
            })
        } else {
            statistics.set(empty_statistics);
            failure.set(failureString(res));
        }
    }

    function failureString(res) {
        return `Failed to load ${url} with ${ res.statusText }`;
    }
</script>

<Styles/>

<div class="container">
<h1>Node Statistics</h1>

<Card>
{#each $statistics.nodes as {name, stats}}
    <CardHeader>
        <CardTitle>{ name }</CardTitle>
    </CardHeader>
    <CardBody>
        <NodeStats nodeStats={ stats } />
    </CardBody>
{/each}
</Card>

{#if $failure.length > 0}
<Alert color="danger" dismissible>{ failure }</Alert>
{/if}

<Button on:click={ refresh }>Refresh</Button>
</div>
