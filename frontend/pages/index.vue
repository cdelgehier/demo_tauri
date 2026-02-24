<template>
  <div class="min-h-screen bg-gray-50 flex items-center justify-center p-4">
    <div class="bg-white rounded-2xl shadow-lg p-8 w-full max-w-md">
      <h1 class="text-2xl font-bold text-gray-800 mb-8 text-center">
        Sum Calculator
      </h1>

      <form @submit.prevent="handleSubmit" class="space-y-6">
        <div class="flex items-end gap-4">
          <div class="flex-1">
            <label
              for="input-a"
              class="block text-sm font-medium text-gray-600 mb-1"
            >
                Number A
            </label>
            <input
              id="input-a"
              v-model.number="a"
              type="number"
              class="w-full border border-gray-300 rounded-lg px-3 py-2.5 text-center text-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              placeholder="0"
            />
          </div>

          <span class="text-3xl font-light text-gray-400 pb-2">+</span>

          <div class="flex-1">
            <label
              for="input-b"
              class="block text-sm font-medium text-gray-600 mb-1"
            >
                Number B
            </label>
            <input
              id="input-b"
              v-model.number="b"
              type="number"
              class="w-full border border-gray-300 rounded-lg px-3 py-2.5 text-center text-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              placeholder="0"
            />
          </div>
        </div>

        <button
          type="submit"
          :disabled="loading"
          class="w-full bg-blue-600 hover:bg-blue-700 disabled:bg-blue-300 text-white font-semibold py-3 rounded-lg transition-colors duration-150 cursor-pointer disabled:cursor-not-allowed"
        >
            {{ loading ? "Calculating…" : "Calculate" }}
        </button>
      </form>

      <Transition
        enter-active-class="transition-all duration-300 ease-out"
        enter-from-class="opacity-0 translate-y-2"
        enter-to-class="opacity-100 translate-y-0"
      >
        <div
          v-if="result !== null"
          class="mt-6 p-5 bg-green-50 border border-green-100 rounded-xl text-center"
        >
            <p class="text-sm text-gray-500 mb-1">Result</p>
          <p class="text-5xl font-bold text-green-600">{{ result }}</p>
        </div>
      </Transition>

      <Transition
        enter-active-class="transition-all duration-200 ease-out"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
      >
        <div
          v-if="error"
          class="mt-4 p-3 bg-red-50 border border-red-100 rounded-lg text-red-600 text-sm text-center"
        >
          {{ error }}
        </div>
      </Transition>
    </div>
  </div>
</template>

<script setup lang="ts">
const a = ref<number>(0);
const b = ref<number>(0);
const { result, error, loading, compute } = useSum();

async function handleSubmit() {
  await compute(a.value, b.value);
}
</script>
