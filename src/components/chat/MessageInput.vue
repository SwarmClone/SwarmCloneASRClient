<script setup lang="ts">

</script>

<template>
  <form class="flex w-full items-center rounded-b-md border-t border-slate-300 bg-slate-200 p-2 dark:border-slate-700 dark:bg-slate-900">
    <label for="chat" class="sr-only">Enter your prompt</label>
    <div>
      <button class="hover:text-blue-600 dark:hover:text-blue-600 sm:p-2" type="button">
        <!-- 加号图标 -->
      </button>
    </div>
    <textarea
      id="chat-input"
      rows="1"
      class="mx-2 min-h-full w-full rounded-md border border-slate-300 bg-slate-50 p-2 text-base placeholder-slate-400 focus:border-blue-600 focus:ring-blue-600 dark:border-slate-700 dark:bg-slate-800 dark:placeholder-slate-400 dark:focus:border-blue-600 dark:focus:ring-blue-600"
      placeholder="Enter your prompt"
    ></textarea>
    <div>
      <button class="inline-flex hover:text-blue-600 dark:hover:text-blue-600 sm:p-2" type="submit">
        <!-- 发送图标 -->
      </button>
    </div>
  </form>
</template>