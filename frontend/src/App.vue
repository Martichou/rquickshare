<script setup lang="ts">
import { Suspense } from 'vue';
import Home from './components/Home.vue'
</script>

<template>
	<div class="w-screen h-screen">
		<Suspense>
			<Home />
		</Suspense>
	</div>
</template>
