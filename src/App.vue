<script setup lang="ts"></script>

<template>
  <div id="top-container">
    <h1 class="app-title">Encrypted File Access</h1>
    <div class="button-list">
      <button>Create</button>
      <button>Open</button>
    </div>
  </div>
</template>

<style>
html {
  height: 100%;
}

body {
  user-select: none;

  height: stretch;
  margin-top: 0;

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
