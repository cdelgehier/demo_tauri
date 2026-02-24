<template>
  <div v-if="timedOut" class="min-h-screen bg-gray-50 flex items-center justify-center p-4">
    <div class="text-center space-y-4">
      <p class="text-5xl">⚠️</p>
      <p class="text-lg text-gray-700 font-medium">Something went wrong</p>
      <p class="text-sm text-gray-500">The application could not start. Please try again.</p>
    </div>
  </div>

  <div v-else-if="!ready" class="min-h-screen bg-gray-50 flex items-center justify-center p-4">
    <div class="text-center space-y-4">
      <div class="inline-block h-10 w-10 animate-spin rounded-full border-4 border-blue-500 border-t-transparent" />
      <p class="text-lg text-gray-600 font-medium">Starting up…</p>
    </div>
  </div>

  <NuxtPage v-else />
</template>

<script setup lang="ts">
const { ready, timedOut } = useBackendReady();
</script>
