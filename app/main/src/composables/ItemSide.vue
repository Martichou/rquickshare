<script setup lang="ts">
import { DisplayedItem } from '../vue_lib';
import { utils } from '../vue_lib';
import { PropType } from 'vue';

defineProps({
	item: {
		type: Object as PropType<DisplayedItem>,
		required: true
	}
});
</script>

<template>
	<div>
		<div class="relative w-[62px] h-[62px]">
			<svg
				v-if="item.ack_bytes" width="62" height="62" viewBox="0 0 250 250"
				class="circular-progress" :style="utils.getProgress(item)"
				:class="{'error': item.state && ['Cancelled', 'Rejected', 'Disconnected'].includes(item.state)}">
				<circle class="bg" />
				<circle class="fg" />
			</svg>
			<div class="h-14 w-14 rounded-full bg-white dark:bg-slate-800 absolute top-0 left-0 bottom-0 right-0 m-auto">
				<svg
					v-if="item.state === 'Finished'" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
					width="24" class="w-full h-full p-4 fill-current">
					<!-- eslint-disable-next-line -->
                    <path d="M268-240 42-466l57-56 170 170 56 56-57 56Zm226 0L268-466l56-57 170 170 368-368 56 57-424 424Zm0-226-57-56 198-198 57 56-198 198Z" />
				</svg>
				<svg
					v-else-if="item.deviceType === 'Laptop'" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
					width="24" class="w-full h-full p-4 fill-current">
					<!-- eslint-disable-next-line -->
                    <path d="M0-160v-80h160v-40q-33 0-56.5-23.5T80-360v-400q0-33 23.5-56.5T160-840h640q33 0 56.5 23.5T880-760v400q0 33-23.5 56.5T800-280v40h160v80H0Zm160-200h640v-400H160v400Zm0 0v-400 400Z" />
				</svg>
				<svg
					v-else-if="item.deviceType === 'Phone'" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
					width="24" class="w-full h-full p-4 fill-current">
					<!-- eslint-disable-next-line -->
                    <path d="M280-40q-33 0-56.5-23.5T200-120v-720q0-33 23.5-56.5T280-920h400q33 0 56.5 23.5T760-840v720q0 33-23.5 56.5T680-40H280Zm0-120v40h400v-40H280Zm0-80h400v-480H280v480Zm0-560h400v-40H280v40Zm0 0v-40 40Zm0 640v40-40Z" />
				</svg>
				<svg
					v-else-if="item.deviceType === 'Tablet'" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
					width="24" class="w-full h-full p-4 fill-current">
					<!-- eslint-disable-next-line -->
                    <path d="M120-160q-33 0-56.5-23.5T40-240v-480q0-33 23.5-56.5T120-800h720q33 0 56.5 23.5T920-720v480q0 33-23.5 56.5T840-160H120Zm40-560h-40v480h40v-480Zm80 480h480v-480H240v480Zm560-480v480h40v-480h-40Zm0 0h40-40Zm-640 0h-40 40Z" />
				</svg>
				<svg
					v-else xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
					width="24" class="w-full h-full p-4 fill-current">
					<!-- eslint-disable-next-line -->
                    <path d="M280-160H160q-33 0-56.5-23.5T80-240v-480q0-33 23.5-56.5T160-800h640v80H160v480h120v80Zm160-100q25 0 42.5-17.5T500-320q0-25-17.5-42.5T440-380q-25 0-42.5 17.5T380-320q0 25 17.5 42.5T440-260Zm-80 100v-71q-19-17-29.5-40T320-320q0-26 10.5-49t29.5-40v-71h160v71q19 17 29.5 40t10.5 49q0 26-10.5 49T520-231v71H360Zm480 0H640q-17 0-28.5-11.5T600-200v-360q0-17 11.5-28.5T640-600h200q17 0 28.5 11.5T880-560v360q0 17-11.5 28.5T840-160Zm-160-80h120v-280H680v280Zm0 0h120-120Z" />
				</svg>
			</div>
		</div>

		<p
			v-if="(item.state === 'WaitingForUserConsent' || item.state === 'SentIntroduction') && item.pin_code"
			class="text-center inline-flex gap-1 mt-4 text-sm items-center">
			<svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" class="fill-current">
				<!-- eslint-disable-next-line -->
                <path d="M420-360h120l-23-129q20-10 31.5-29t11.5-42q0-33-23.5-56.5T480-640q-33 0-56.5 23.5T400-560q0 23 11.5 42t31.5 29l-23 129Zm60 280q-139-35-229.5-159.5T160-516v-244l320-120 320 120v244q0 152-90.5 276.5T480-80Zm0-84q104-33 172-132t68-220v-189l-240-90-240 90v189q0 121 68 220t172 132Zm0-316Z" />
			</svg>
			{{ item.pin_code }}
		</p>
	</div>
</template>