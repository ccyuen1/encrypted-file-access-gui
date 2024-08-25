<script setup lang="ts">
const emit = defineEmits<{ "switch-page": [page: string] }>();
</script>

<template>
  <div id="top-container">
    <h1 class="app-title">Encrypted File Access</h1>
    <div class="button-list">
      <button @click="emit('switch-page', 'CreatePage')">Create</button>
      <button @click="emit('switch-page', 'OpenPage')">Open</button>
    </div>
  </div>
</template>

<style scoped>
#top-container {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.app-title {
  text-align: center;
}

.button-list {
  display: flex;
  flex-direction: column;
  row-gap: 0.5rem;
  height: 100%;
}

.button-list button {
  flex-grow: 1;
  font-size: 20px;
}
</style>
