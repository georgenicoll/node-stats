<script>
    import { Styles } from 'sveltestrap';

	export let nodeStats;
</script>

<Styles/>

<div class="container">
	<ul>
		{#each nodeStats as { key, value }}
			<li>{key}: {value}</li>
		{/each}
	</ul>
</div>