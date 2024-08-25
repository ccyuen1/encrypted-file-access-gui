<script setup lang="ts">
const emit = defineEmits<{ "goto-menu": [] }>();
</script>

<template>
  <div>
    <nav><button @click="emit('goto-menu')">&larr; Go Back</button></nav>
    <div>update password</div>
  </div>
</template>
