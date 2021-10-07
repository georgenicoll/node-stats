<script>
    export let nodeName;
    export let nodeStats;
</script>

<h1>{ nodeName }</h1>
<ul>
    {#each nodeStats as { key, value }}
    <li>{ key }: { value }</li>
    {/each}
</ul>