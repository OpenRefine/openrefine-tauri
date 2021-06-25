<script>
	import { onMount } from 'svelte'
	import { writable } from 'svelte/store'
	import { Command } from '@tauri-apps/api/shell'
	import { resourceDir } from '@tauri-apps/api/path'

	const output = writable([])

	onMount(async () => {
		const resourcePath = await resourceDir()
		const cmd = Command.sidecar('refine', ['-w', `${resourcePath}/resources/webapp`], {
			env: {
				REFINE_LIB_DIR: `${resourcePath}/resources/server/target/lib`
			}
		})
		cmd.stdout.on('data', line => {
			output.update(out => [...out, line])
		})
		cmd.stderr.on('data', line => {
			output.update(out => [...out, line])
		})
		cmd.on('error', console.error)
		cmd.on('close', console.log)
		cmd.spawn().catch(console.error)
	})
</script>

<main>
	<div class="terminal">
		{#each $output as line}
			<p>{line}</p>
		{/each}
	</div>
</main>

<style>
.terminal {
	font-size: 14px;
}
</style>
