Update(app/main/src/vue_lib/assets/main.postcss)
  ⎿  Added 1 line                                                                                                                                                
      24    transform: translateZ(0);
      25    will-change: transform;
      26    -webkit-perspective: 1000;
      27 +  @apply text-black dark:text-gray-100;
      28  }
      29 
      30  .status-indicator {

● Update(app/main/src/vue_lib/assets/main.postcss)
  ⎿  Added 3 lines, removed 3 lines                                                                                                                              
      67 
      68  .status-indicator .circle-main {
      69      z-index: 3;
      70 -    background: #000                                                                                                                                   
      70 +    @apply bg-black dark:bg-white;
      71  }
      72 
      73  .status-indicator .circle-main.circle--animated {
     ...
      76 
      77  .status-indicator .circle-secondary {
      78      z-index: 2;
      79 -    background: rgba(0, 0, 0, .1)                                                                                                                      
      79 +    @apply bg-black bg-opacity-10 dark:bg-white dark:bg-opacity-10;
      80  }
      81 
      82  .status-indicator .circle-secondary.circle--animated {
     ...
      85 
      86  .status-indicator .circle-tertiary {
      87      z-index: 1;
      88 -    background: rgba(0, 0, 0, .6);                                                                                                                     
      88 +    @apply bg-black bg-opacity-60 dark:bg-white dark:bg-opacity-60;
      89      opacity: 1
      90  }
      91 

● Update(app/main/src/vue_lib/assets/main.postcss)
  ⎿  Added 3 lines, removed 3 lines                                                                                                                              
       98  }
       99 
      100  .status-indicator--success .circle-main {
      101 -  @apply bg-black                                                                                                                                     
      101 +  @apply bg-black dark:bg-white
      102  }
      103 
      104  .status-indicator--success .circle-secondary {
      105 -  @apply bg-black bg-opacity-10                                                                                                                       
      105 +  @apply bg-black bg-opacity-10 dark:bg-white dark:bg-opacity-10
      106  }
      107 
      108  .status-indicator--success .circle-tertiary {
      109 -  @apply bg-black bg-opacity-40                                                                                                                       
      109 +  @apply bg-black bg-opacity-40 dark:bg-white dark:bg-opacity-40
      110  }
      111 
      112  @keyframes pulsate-tertiary {

● Update(app/main/src/vue_lib/assets/main.postcss)
  ⎿  Added 1 line, removed 1 line                                                                                                                                
      157  }
      158 
      159  .btn {
      160 -  @apply h-12 min-h-12 cursor-pointer rounded-xl font-medium px-4 inline-flex justify-center items-center gap-2 hover:bg-gray-500 hover:bg-opacity-10 
      160 +  @apply h-12 min-h-12 cursor-pointer rounded-xl font-medium px-4 inline-flex justify-center items-center gap-2 hover:bg-gray-500 hover:bg-opacity-10
          +dark:hover:bg-white dark:hover:bg-opacity-10
      161  }
      162 
      163  .checkbox {

● Update(app/main/src/vue_lib/assets/main.postcss)
  ⎿  Added 2 lines, removed 2 lines                                                                                                                              
      206  }
      207 
      208  .circular-progress circle.bg {
      209 -  	stroke: #ddd;                                                                                                                                      


      209 +  	@apply stroke-gray-300 dark:stroke-gray-600;


      210  }
      211 
      212  .circular-progress circle.fg {
     ...
      215    stroke-dasharray: var(--dash) calc(var(--circumference) - var(--dash));
      216    transition: stroke-dasharray 0.3s linear 0s;
      217 
      218 -  @apply stroke-black                                                                                                                                 
      218 +  @apply stroke-black dark:stroke-white
      219  }
      220 
      221  .error circle.bg, .error circle.fg {

● Update(app/main/src/components/HomePage.vue)
  ⎿  Added 3 lines, removed 3 lines                                                                                                                              
      1  <template>
      2 -  <div class="flex flex-col w-full h-full bg-green-50 max-w-full max-h-full overflow-hidden">                                                           
      2 +  <div class="flex flex-col w-full h-full bg-green-50 dark:bg-slate-900 max-w-full max-h-full overflow-hidden">
      3      <ToastNotification />
      4      <SettingsModal :vm="vm" @close="settingsOpen = false" />
      5 
     ...
       8      <div class="flex-1 flex flex-row">
       9        <SideMenu :vm="vm" @invert-visibility="invertVisibility(vm)" @clear-sending="clearSending(vm)" />
      10 
      11 -      <div class="flex-1 flex flex-col bg-white w-full max-w-full min-w-0 min-h-full rounded-tl-[3rem] p-12 h-1 overflow-y-scroll">                    
      11 +      <div class="flex-1 flex flex-col bg-white dark:bg-slate-800 w-full max-w-full min-w-0 min-h-full rounded-tl-[3rem] p-12 h-1 overflow-y-scroll">
      12          <ContentStatus :vm="vm" @outbound-payload="(el: OutboundPayload) => outboundPayload = el" @discovery-running="discoveryRunning = true;" />
      13 
      14          <div
      15 -          v-for="item in displayedItems" :key="item.id" class="w-full rounded-3xl flex flex-row gap-6 p-4 mb-4 bg-green-100"                           
      15 +          v-for="item in displayedItems" :key="item.id" class="w-full rounded-3xl flex flex-row gap-6 p-4 mb-4 bg-green-100 dark:bg-slate-700"
      16            :class="{'cursor-pointer': item.endpoint}" @click="item.endpoint && sendInfo(vm, item.id)">
      17            <!-- Loader and image of the device type & pin_code -->
      18            <ItemSide :item="item" />

● Update(app/main/src/composables/SideMenu.vue)
  ⎿  Added 2 lines, removed 2 lines                                                                                                                              
      55        <p class="mt-4 mb-2">
      56          Sharing {{ props.vm.outboundPayload.Files.length }} {{ pluralize(props.vm.outboundPayload.Files.length, "file") }}
      57        </p>
      58 -      <div class="bg-white w-32 h-32 rounded-2xl mb-2 flex justify-center items-center">                                                               
      58 +      <div class="bg-white dark:bg-slate-800 w-32 h-32 rounded-2xl mb-2 flex justify-center items-center">
      59          <svg
      60            xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"
      61 -          class="w-8 h-8">                                                                                                                             
      61 +          class="w-8 h-8 fill-current">
      62            <!-- eslint-disable-next-line -->
      63                      <path d="M240-80q-33 0-56.5-23.5T160-160v-640q0-33 23.5-56.5T240-880h320l240 240v480q0 33-23.5 56.5T720-80H240Zm280-520v-200H240v64
          0h480v-440H520ZM240-800v200-200 640-640Z" />
      64          </svg>

● Update(app/main/src/composables/SideMenu.vue)
  ⎿  Added 1 line, removed 1 line                                                                                                                                
      28 
      29        <svg
      30          xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24"
      31 -        :class="{'rotate-180': props.vm.visibility === 'Invisible'}">                                                                                  
      31 +        :class="{'rotate-180': props.vm.visibility === 'Invisible'}" class="fill-current">
      32          <path d="M504-480 320-664l56-56 240 240-240 240-56-56 184-184Z" />
      33        </svg>
      34      </h4>

● Update(app/main/src/composables/Heading.vue)
  ⎿  Added 1 line, removed 1 line                                                                                                                                
      46        <div class="btn px-3 rounded-xl active:scale-95 transition duration-150 ease-in-out" @click="emit('openSettings')">
      47          <svg
      48            xmlns="http://www.w3.org/2000/svg" height="24"
      49 -          viewBox="0 -960 960 960" width="24">                                                                                                         
      49 +          viewBox="0 -960 960 960" width="24" class="fill-current">
      50            <!-- eslint-disable-next-line -->
      51              <path d="m370-80-16-128q-13-5-24.5-12T307-235l-119 50L78-375l103-78q-1-7-1-13.5v-27q0-6.5 1-13.5L78-585l110-190 119 50q11-8 23-15t24-12l16-
          128h220l16 128q13 5 24.5 12t22.5 15l119-50 110 190-103 78q1 7 1 13.5v27q0 6.5-2 13.5l103 78-110 190-118-50q-11 8-23 15t-24 12L590-80H370Zm70-80h79l14-1
          06q31-8 57.5-23.5T639-327l99 41 39-68-86-65q5-14 7-29.5t2-31.5q0-16-2-31.5t-7-29.5l86-65-39-68-99 42q-22-23-48.5-38.5T533-694l-13-106h-79l-14 106q-31 8
          -57.5 23.5T321-633l-99-41-39 68 86 64q-5 15-7 30t-2 32q0 16 2 31t7 30l-86 65 39 68 99-42q22 23 48.5 38.5T427-266l13 106Zm42-180q58 0 99-41t41-99q0-58-4
          1-99t-99-41q-59 0-99.5 41T342-480q0 58 40.5 99t99.5 41Zm-2-140Z"/>
      52          </svg>

● Update(app/main/src/composables/ContentStatus.vue)
  ⎿  Added 3 lines, removed 3 lines                                                                                                                              
      56 
      57    <div
      58      v-if="props.vm.displayedIsEmpty && props.vm.outboundPayload === undefined" class="w-full border
      59 -        rounded-2xl p-6 flex flex-col justify-center items-center transition duration-150 ease-in-out mt-auto"                                         
      60 -    :class="{'border-green-200 bg-green-100 scale-105': props.vm.isDragHovering}">                                                                     
      59 +        rounded-2xl p-6 flex flex-col justify-center items-center transition duration-150 ease-in-out mt-auto dark:border-slate-600"
      60 +    :class="{'border-green-200 bg-green-100 dark:bg-slate-700 scale-105': props.vm.isDragHovering}">
      61      <svg
      62        xmlns="http://www.w3.org/2000/svg" height="24"
      63 -      viewBox="0 -960 960 960" width="24" class="w-8 h-8">                                                                                             
      63 +      viewBox="0 -960 960 960" width="24" class="w-8 h-8 fill-current">
      64        <!-- eslint-disable-next-line -->
      65              <path d="M440-320v-326L336-542l-56-58 200-200 200 200-56 58-104-104v326h-80ZM240-160q-33 0-56.5-23.5T160-240v-120h80v120h480v-120h80v120q0 
          33-23.5 56.5T720-160H240Z" />
      66      </svg>

● Update(app/main/src/composables/ContentStatus.vue)
  ⎿  Added 1 line, removed 1 line                                                                                                                                
      68        Drop files to send
      69      </h4>
      70      <div class="btn mt-2 active:scale-95 transition duration-150 ease-in-out" @click="openFilePicker()">
      71 -      <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24">                                                         
      71 +      <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" class="fill-current">
      72          <path d="M440-440H200v-80h240v-240h80v240h240v80H520v240h-80v-240Z" />
      73        </svg>
      74        <span class="ml-2">Select</span>

● Update(app/main/src/composables/ItemSide.vue)
  ⎿  Added 6 lines, removed 6 lines                                                                                                                              
      21          <circle class="bg" />
      22          <circle class="fg" />
      23        </svg>
      24 -      <div class="h-14 w-14 rounded-full bg-white absolute top-0 left-0 bottom-0 right-0 m-auto">                                                      
      24 +      <div class="h-14 w-14 rounded-full bg-white dark:bg-slate-800 absolute top-0 left-0 bottom-0 right-0 m-auto">
      25          <svg
      26            v-if="item.state === 'Finished'" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
      27 -          width="24" class="w-full h-full p-4">                                                                                                        
      27 +          width="24" class="w-full h-full p-4 fill-current">
      28            <!-- eslint-disable-next-line -->
      29                      <path d="M268-240 42-466l57-56 170 170 56 56-57 56Zm226 0L268-466l56-57 170 170 368-368 56 57-424 424Zm0-226-57-56 198-198 57 56-19
          8 198Z" />
      30          </svg>
      31          <svg
      32            v-else-if="item.deviceType === 'Laptop'" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
      33 -          width="24" class="w-full h-full p-4">                                                                                                        
      33 +          width="24" class="w-full h-full p-4 fill-current">
      34            <!-- eslint-disable-next-line -->
      35                      <path d="M0-160v-80h160v-40q-33 0-56.5-23.5T80-360v-400q0-33 23.5-56.5T160-840h640q33 0 56.5 23.5T880-760v400q0 33-23.5 56.5T800-28
          0v40h160v80H0Zm160-200h640v-400H160v400Zm0 0v-400 400Z" />
      36          </svg>
      37          <svg
      38            v-else-if="item.deviceType === 'Phone'" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
      39 -          width="24" class="w-full h-full p-4">                                                                                                        
      39 +          width="24" class="w-full h-full p-4 fill-current">
      40            <!-- eslint-disable-next-line -->
      41                      <path d="M280-40q-33 0-56.5-23.5T200-120v-720q0-33 23.5-56.5T280-920h400q33 0 56.5 23.5T760-840v720q0 33-23.5 56.5T680-40H280Zm0-12
          0v40h400v-40H280Zm0-80h400v-480H280v480Zm0-560h400v-40H280v40Zm0 0v-40 40Zm0 640v40-40Z" />
      42          </svg>
      43          <svg
      44            v-else-if="item.deviceType === 'Tablet'" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
      45 -          width="24" class="w-full h-full p-4">                                                                                                        
      45 +          width="24" class="w-full h-full p-4 fill-current">
      46            <!-- eslint-disable-next-line -->
      47                      <path d="M120-160q-33 0-56.5-23.5T40-240v-480q0-33 23.5-56.5T120-800h720q33 0 56.5 23.5T920-720v480q0 33-23.5 56.5T840-160H120Zm40-
          560h-40v480h40v-480Zm80 480h480v-480H240v480Zm560-480v480h40v-480h-40Zm0 0h40-40Zm-640 0h-40 40Z" />
      48          </svg>
      49          <svg
      50            v-else xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960"
      51 -          width="24" class="w-full h-full p-4">                                                                                                        
      51 +          width="24" class="w-full h-full p-4 fill-current">
      52            <!-- eslint-disable-next-line -->
      53                      <path d="M280-160H160q-33 0-56.5-23.5T80-240v-480q0-33 23.5-56.5T160-800h640v80H160v480h120v80Zm160-100q25 0 42.5-17.5T500-320q0-25
          -17.5-42.5T440-380q-25 0-42.5 17.5T380-320q0 25 17.5 42.5T440-260Zm-80 100v-71q-19-17-29.5-40T320-320q0-26 10.5-49t29.5-40v-71h160v71q19 17 29.5 40t10.
          5 49q0 26-10.5 49T520-231v71H360Zm480 0H640q-17 0-28.5-11.5T600-200v-360q0-17 11.5-28.5T640-600h200q17 0 28.5 11.5T880-560v360q0 17-11.5 28.5T840-160Zm
          -160-80h120v-280H680v280Zm0 0h120-120Z" />
      54          </svg>

● Update(app/main/src/composables/ItemSide.vue)
  ⎿  Added 1 line, removed 1 line                                                                                                                                
      58      <p
      59        v-if="(item.state === 'WaitingForUserConsent' || item.state === 'SentIntroduction') && item.pin_code"
      60        class="text-center inline-flex gap-1 mt-4 text-sm items-center">
      61 -      <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24">                                                         
      61 +      <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" class="fill-current">
      62          <!-- eslint-disable-next-line -->
      63                  <path d="M420-360h120l-23-129q20-10 31.5-29t11.5-42q0-33-23.5-56.5T480-640q-33 0-56.5 23.5T400-560q0 23 11.5 42t31.5 29l-23 129Zm60 280
          q-139-35-229.5-159.5T160-516v-244l320-120 320 120v244q0 152-90.5 276.5T480-80Zm0-84q104-33 172-132t68-220v-189l-240-90-240 90v189q0 121 68 220t172 132Z
          m0-316Z" />
      64        </svg>

● Read(app/main/src/vue_lib/components/atoms/ToastMessage.vue)
  ⎿  Read 29 lines

● Read(app/main/src/vue_lib/components/organisms/ToastNotification.vue)
  ⎿  Read 26 lines

● I also noticed ContentStatus.vue has a conditional class for the drag hover state:
  :class="{'border-green-200 bg-green-100 dark:bg-slate-700 scale-105': props.vm.isDragHovering}">
  I changed bg-green-100 to bg-green-100 dark:bg-slate-700.
  Wait, if it is drag hovering, it should probably be highlighted.
  In dark mode, bg-slate-700 is the same color as the items.
  Maybe I should use a slightly different color for hover state in dark mode, like dark:bg-slate-600 or dark:bg-slate-700 (if the default is dark:border-slate-600 and transparent/default background).
  The default background for ContentStatus container is bg-white dark:bg-slate-800.
  So if I drag hover, dark:bg-slate-700 is lighter/different than slate-800. That seems ok.

  Let's modify ToastMessage.vue.

● Update(app/main/src/vue_lib/components/atoms/ToastMessage.vue)
  ⎿  Added 6 lines, removed 2 lines                                                                                                                              
      21  <template>
      22    <div
      23      class="w-full p-4 rounded-xl shadow-md"
      24 -    :class="{'bg-red-100': type === ToastType.Error, 'bg-green-100': type === ToastType.Success, 'bg-blue-100': type === ToastType.Info}">             
      25 -    <span class="text-sm text-gray-800">                                                                                                               
      24 +    :class="{
      25 +      'bg-red-100 dark:bg-red-900': type === ToastType.Error,
      26 +      'bg-green-100 dark:bg-green-900': type === ToastType.Success,
      27 +      'bg-blue-100 dark:bg-blue-900': type === ToastType.Info
      28 +    }">
      29 +    <span class="text-sm text-gray-800 dark:text-gray-100">
      30        {{ message }}
      31      </span>
      32    </div>

● Update(app/main/src/vue_lib/assets/main.postcss)
  ⎿  Added 1 line, removed 1 line                                                                                                                                
      157  }
      158 
      159  .btn {
      160 -  @apply h-12 min-h-12 cursor-pointer rounded-xl font-medium px-4 inline-flex justify-center items-center gap-2 hover:bg-gray-500 hover:bg-opacity-10 
          -dark:hover:bg-white dark:hover:bg-opacity-10                                                                                                          
      160 +  @apply h-12 min-h-12 cursor-pointer rounded-xl font-medium px-4 inline-flex justify-center items-center gap-2 hover:bg-gray-500 hover:bg-opacity-10
          +dark:bg-white dark:bg-opacity-5 dark:hover:bg-opacity-10 transition-colors
      161  }
      162 
      163  .checkbox {

● Update(app/main/src/vue_lib/assets/main.postcss)
  ⎿  Added 1 line                                                                                                                                                
      174      border-width: 1px;
      175      border-color: var(--fallback-bc, oklch(var(--bc) / var(--tw-border-opacity)));
      176      --tw-border-opacity: .2;
      177 +    @apply dark:border-gray-500;
      178  }
      179 
      180  .checkbox:checked, .checkbox[checked=true], .checkbox[aria-checked=true] {

● Update(app/main/src/composables/ContentStatus.vue)
  ⎿  Added 1 line, removed 1 line                                                                                                                                
      57    <div
      58      v-if="props.vm.displayedIsEmpty && props.vm.outboundPayload === undefined" class="w-full border
      59          rounded-2xl p-6 flex flex-col justify-center items-center transition duration-150 ease-in-out mt-auto dark:border-slate-600"
      60 -    :class="{'border-green-200 bg-green-100 dark:bg-slate-700 scale-105': props.vm.isDragHovering}">                                                   
      60 +    :class="{'border-green-200 dark:border-green-700 bg-green-100 dark:bg-slate-700 scale-105': props.vm.isDragHovering}">
      61      <svg
      62        xmlns="http://www.w3.org/2000/svg" height="24"
      63        viewBox="0 -960 960 960" width="24" class="w-8 h-8 fill-current">

● Read(app/main/index.html)
  ⎿  Read 14 lines

● Read(app/main/src/vue_lib/assets/main.postcss)
  ⎿  Read 224 lines

● Update(app/main/src/vue_lib/assets/main.postcss)
  ⎿  Added 1 line, removed 1 line                                                                                                                                
      24    transform: translateZ(0);
      25    will-change: transform;
      26    -webkit-perspective: 1000;
      27 -  @apply text-black dark:text-gray-100;                                                                                                                
      27 +  @apply text-black dark:text-gray-100 dark:bg-slate-900;
      28  }
      29 
      30  .status-indicator {

● Update(app/main/src/composables/SettingsModal.vue)
  ⎿  Added 1 line, removed 1 line                                                                                                                                
      61            <label class="cursor-pointer flex flex-row justify-between items-center">
      62              <span class="label-text">Theme</span>
      63              <select
      64 -              class="bg-transparent text-right focus:outline-none cursor-pointer dark:bg-slate-800"                                                    
      64 +              class="bg-transparent text-right focus:outline-none cursor-pointer"
      65                :value="vm.theme"
      66                @change="utils.setTheme(vm, ($event.target as HTMLSelectElement).value as any)"
      67              >

● Update(app/main/src/vue_lib/assets/main.postcss)
  ⎿  Added 4 lines                                                                                                                                               
      27    @apply text-black dark:text-gray-100 dark:bg-slate-900;
      28  }
      29 
      30 +html.dark {
      31 +  color-scheme: dark;
      32 +}
      33 +
      34  .status-indicator {
      35      position: relative;
      36      display: inline-block;
