// <script lang="ts">
// 	import Tooth from './Tooth.svelte'

// 	export let side: 'left' | 'right' = 'left'
// 	export let last = false
// </script>

// <section
// 	class="
// 		relative flex flex-row
// 		px-grid-6
// 		w-full
// 		{last ? 'h-grid-52' : 'h-grid-68'}
// 	"
// 	class:justify-start={side === 'left'}
// 	class:justify-end={side === 'right'}
// >
// 	<Tooth {side} {last} class="absolute top-0 {side === 'left' ? 'right-0' : 'left-0'}" />

// 	<div class="flex flex-col w-grid-93" class:items-end={side === 'right'}>
// 		<slot />
// 	</div>
// </section>
