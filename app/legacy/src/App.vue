<script setup lang="ts">
import HomePage from './components/HomePage.vue'
</script>

<template>
	<div class="w-screen h-screen select-none cursor-default">
		<Suspense>
			<HomePage />
		</Suspense>
	</div>
</template>
