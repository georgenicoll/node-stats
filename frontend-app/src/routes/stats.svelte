<script>
	import NodeStats from '$lib/NodeStats.svelte';
	import { onMount } from 'svelte';
	import { writable } from 'svelte/store';
	import {
		Styles,
		Card,
		CardHeader,
		CardTitle,
		CardBody,
		Button,
		Spinner,
		Fade,
		Collapse
	} from 'sveltestrap';

	const empty_statistics = { loading: false, nodes: [] };
	const loading_statistics = { loading: true, nodes: [] };
	const empty_failure = { failed: false, short_message: '', long_message: '' };
	const url = `/statistics.json`;

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
						return a.key.localeCompare(b.key);
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

<Styles />

<div class="container">
	<h1>Node Statistics</h1>

	{#if !$failure.failed}
		<Collapse isOpen={$statistics.loading}>
			<Fade isOpen={$statistics.loading}>
				<Spinner color="info" />
			</Fade>
		</Collapse>
		<Collapse isOpen={!$statistics.loading}>
			<Fade isOpen={!$statistics.loading}>
				<Card body color="light">
					{#each $statistics.nodes as { name, stats }}
						<CardHeader>
							<CardTitle>{name}</CardTitle>
						</CardHeader>
						<CardBody>
							<NodeStats nodeStats={stats} />
						</CardBody>
					{/each}
				</Card>
			</Fade>
		</Collapse>
	{/if}

	{#if $failure.failed}
		<Card body color="danger">
			<CardHeader>
				<CardTitle>{$failure.short_message}</CardTitle>
			</CardHeader>
			<CardBody>{$failure.long_message}</CardBody>
		</Card>
	{/if}

	<Fade isOpen={!$statistics.loading}>
		<Button on:click={refresh}>Refresh</Button>
	</Fade>
</div>
