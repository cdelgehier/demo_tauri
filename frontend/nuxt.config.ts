export default defineNuxtConfig({
  ssr: false,
  modules: ["@nuxtjs/tailwindcss"],
  runtimeConfig: {
    public: {
      apiBase: "http://127.0.0.1:8000",
    },
  },
  compatibilityDate: "2024-11-01",
});
