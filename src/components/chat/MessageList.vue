<script setup lang="ts">
const messages = [
  {
    type: 'user',
    content: 'Explain quantum computing in simple terms',
    response: {
      content: `Certainly! Quantum computing is a new type of computing that relies on...
        Unlike bits, qubits can represent not only a 0 or a 1 but also a superposition...`,
    }
  },
  {
    type: 'user',
    content: 'What are three great applications of quantum computing?',
    response: {
      content: 'Three great applications of quantum computing are: Optimization of complex problems...',
    }
  }
];
</script>

<template>
  <div class="flex-1 overflow-y-auto bg-slate-300 text-sm leading-6 shadow-md dark:bg-slate-800 sm:text-base sm:leading-7">
    <div v-for="(message, index) in messages" :key="index">
      <div class="flex px-4 py-8 sm:px-6">
        <img
          class="mr-2 h-8 w-8 rounded-full sm:mr-4"
          src="https://dummyimage.com/256x256/363536/ffffff&text=U"
        />
        <div class="flex max-w-3xl items-center">
          <p>{{ message.content }}</p>
        </div>
      </div>

      <div class="bg-slate-100 px-4 py-8 dark:bg-slate-900 sm:px-6">
        <img
          class="mr-2 h-8 w-8 rounded-full sm:mr-4"
          src="https://dummyimage.com/256x256/354ea1/ffffff&text=G"
        />
        <div class="w-full lg:flex lg:justify-between">
          <p class="max-w-3xl">{{ message.response.content }}</p>
          <div class="mt-4 flex gap-x-2 text-slate-500 lg:mt-0">
          </div>
        </div>
      </div>
    </div>
  </div>
</template>