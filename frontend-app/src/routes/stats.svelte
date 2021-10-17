<script lang="ts">
	import NodeStats from '$lib/NodeStats.svelte';
	import { onMount } from 'svelte';
	import { writable } from 'svelte/store';
	import TopAppBar, {
		Row,
		Section,
		Title,
		AutoAdjust,
		TopAppBarComponentDev
	} from '@smui/top-app-bar';
	import IconButton, { Icon } from '@smui/icon-button';
	// import Section from '@smui/common/';
	import Card, { Content } from '@smui/card';
	// --> LayoutGrid
	import LayoutGrid, { Cell } from '@smui/layout-grid';
	import CircularProgress from '@smui/circular-progress';

	let topAppBar: TopAppBarComponentDev;

	const empty_statistics = { loading: false, nodes: [] };
	const loading_statistics = { loading: true, nodes: [] };
	const empty_failure = { failed: false, short_message: '', long_message: '' };
	// const url = `/statistics.json`;
	const url = '/statistics-test-data.json';

	const statistics = writable(loading_statistics);
	const failure = writable(empty_failure);

	onMount(refresh);

	async function refresh() {
		statistics.set(loading_statistics);
		const res = await fetch(url);
		if (res.ok) {
			failure.set(empty_failure);
			await res.json().then((stats) => {
				var obj = {
					loading: false,
					nodes: stats.nodes.sort((a, b) => {
						return a.name.localeCompare(b.name);
					})
				};
				statistics.set(stats);
			});
		} else {
			statistics.set(empty_statistics);
			failure.set(await create_failure_object(res));
		}
	}

	async function create_failure_object(res) {
		return {
			failed: true,
			short_message: `Failed to load ${url} with ${res.status}: ${res.statusText}`,
			long_message: await res.text()
		};
	}
</script>

<svelte:head>
	<title>Stats</title>
</svelte:head>

<TopAppBar bind:this={topAppBar} variant="standard" color="primary">
	<Row>
		<Section>
			<Title>Node Statistics</Title>
		</Section>
		<Section align="end" toolbar>
			{#if !$statistics.loading}
				<IconButton on:click={refresh}>
					<Icon class="material-icons">refresh</Icon>
				</IconButton>
			{/if}
		</Section>
	</Row>
</TopAppBar>

<AutoAdjust {topAppBar}>
	{#if $statistics.loading}
		<div style="display: flex; justify-content: center">
			<CircularProgress style="height: 64px; width: 64px;" on={true} indeterminate={true} />
		</div>
	{/if}
	<div class="container">
		{#if !$failure.failed}
			<!-- Card -->
			<!-- {#each $statistics.nodes as { name, stats }}
				<div class="card-display">
					<div class="card-container">
						<Card padded={false}>
							<Content class="mdc-typography--body2">
								<h2 class="mdc-typography--headline6" style="margin: 0;">
									{name}
								</h2>
								<NodeStats nodeName={name} nodeStats={stats} />
							</Content>
						</Card>
					</div>
				</div>
			{/each} -->
			<!-- Layout Grid -->
			<LayoutGrid>
				{#each $statistics.nodes as { name, stats }}
					<Cell>
						<div class="demo-cell; width: 100%">
							<div class="card-container">
								<Card padded={false}>
									<Content>
										<h2>{name}</h2>
										<NodeStats nodeName={name} nodeStats={stats} />
									</Content>
								</Card>
							</div>
						</div>
					</Cell>
				{/each}
			</LayoutGrid>
		{/if}

		{#if $failure.failed}
			<!-- Card -->
			<!-- <div class="card-container">
				<Card>
					<Content class="mdc-typography--body2">
						<h2 class="mdc-typography--headline6" style="margin: 0;">
							{$failure.short_message}
						</h2>
						<p>{$failure.long_message}</p>
					</Content>
				</Card>
			</div> -->
			<LayoutGrid>
				<Cell>
					<div class="demo-cell">
						<h2 class="mdc-typography--headline6" style="margin: 0;">
							{$failure.short_message}
						</h2>
						{$failure.long_message}
					</div>
				</Cell>
			</LayoutGrid>
		{/if}
	</div>
</AutoAdjust>

<style>
	.demo-cell {
		/* height: 60px; */
		width: auto;
		display: flex;
		justify-content: center;
		align-items: stretch;
		background-color: var(--mdc-theme-secondary, #333);
		color: var(--mdc-theme-on-secondary, #fff);
	}
</style>
