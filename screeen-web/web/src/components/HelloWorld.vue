<template>
	<h1>{{ msg }}</h1>

	<p>
		Recommended IDE setup:
		<a href="https://code.visualstudio.com/" target="_blank">VSCode</a>
		+
		<a
			href="https://marketplace.visualstudio.com/items?itemName=octref.vetur"
			target="_blank"
			>Vetur</a
		>
		or
		<a href="https://github.com/johnsoncodehk/volar" target="_blank"
			>Volar</a
		>
		(if using
		<code>&lt;script setup&gt;</code>)
	</p>

	<p>See <code>README.md</code> for more information.</p>

	<p>
		<a href="https://vitejs.dev/guide/features.html" target="_blank"
			>Vite Docs</a
		>
		|
		<a href="https://v3.vuejs.org/" target="_blank">Vue 3 Docs</a>
	</p>

	<button @click="increment" class="button">
		<i-feather-arrow-up />
	</button>

	<span> count is: {{ count }} </span>

	<button @click="decrement" class="button">
		<i-feather-arrow-down />
	</button>

	<p>
		Edit
		<code>components/HelloWorld.vue</code> to test hot module replacement.
	</p>
</template>

<script setup lang="ts">
import { defineProps, ref } from 'vue';

defineProps({
	msg: {
		type: String,
		required: true,
	},
});

const count = ref(0);
const increment = () => count.value++;
const decrement = () => count.value--;
</script>

<style scoped lang="postcss">
a {
	color: #42b983;
}

label {
	margin: 0 0.5em;
	font-weight: bold;
}

code {
	background-color: #eee;
	padding: 2px 4px;
	border-radius: 4px;
	color: #304455;
}
</style>

<style lang="postcss">
.button {
	@apply m-8 py-2 px-4 rounded-md;
	@apply text-white bg-green-500 hover:bg-green-600;
	@apply transition ease-in-out duration-300;
	@apply focus:outline-none focus:ring-4 focus:ring-green-500 focus:ring-opacity-50;
}
</style>
