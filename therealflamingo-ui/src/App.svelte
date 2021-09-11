<script lang="ts">
	import Router, {location, link} from 'svelte-spa-router';
	import Home from './home.svelte';
	import Krydslistegenerator from './krydsliste/krydslistegenerator.svelte';
	import Navigationbar from "./NavigationBar/navigationbar.svelte";
	export let name: string;
</script>

<main>
	<Navigationbar/>
	<Router routes={{
		'/': Home,
		'/krydslistegenerator': Krydslistegenerator
	}} />
</main>

<style>
	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>