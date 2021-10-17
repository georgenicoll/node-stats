<script lang="ts">
	import DataTable, { Head, Body, Row, Cell } from '@smui/data-table';

	export let nodeName;
	export let nodeStats;
</script>

<DataTable table$aria-label={nodeName} style="width: 100%;">
	<!--Head>
		<Cell>Stat</Cell>
		<Cell>Value</Cell>
	</Head-->
	<Body>
		{#each nodeStats as { key, value }}
			<Row>
				<Cell>{key}</Cell>
				<Cell>{value}</Cell>
			</Row>
		{/each}
	</Body>
</DataTable>
