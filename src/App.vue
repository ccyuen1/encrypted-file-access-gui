<script setup lang="ts">
import { ref } from "vue";
import MenuPage from "./components/MenuPage.vue";
import CreatePage from "./components/CreatePage.vue";
import OpenPage from "./components/OpenPage.vue";
import UpdatePasswordPage from "./components/UpdatePasswordPage.vue";

const page = ref("MenuPage");
</script>

<template>
  <MenuPage v-if="page === 'MenuPage'" @switch-page="(p) => (page = p)" />
  <CreatePage
    v-else-if="page === 'CreatePage'"
    @goto-menu="page = 'MenuPage'"
  />
  <OpenPage v-else-if="page === 'OpenPage'" @goto-menu="page = 'MenuPage'" />
  <UpdatePasswordPage
    v-else-if="page === 'UpdatePasswordPage'"
    @goto-menu="page = 'MenuPage'"
  />
</template>

<style>
html {
  height: 100%;
}

body {
  user-select: none;

  height: stretch;
  margin-top: 0;
  overflow: hidden;

  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

#app {
  height: 100%;
}
</style>
