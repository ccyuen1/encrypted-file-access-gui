<script setup lang="ts">
const emit = defineEmits<{ "goto-menu": [] }>();
</script>

<template>
  <div>
    <button @click="emit('goto-menu')">&larr; Go Back</button>
    <div>open</div>
  </div>
</template>
