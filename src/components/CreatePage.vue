<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { open, save } from "@tauri-apps/api/dialog";
import { ref } from "vue";

const emit = defineEmits<{ "goto-menu": [] }>();

const outFile = ref<string | null>(null);
const srcFile = ref<string | null>(null);
const ext = ref("txt");
const compress = ref(true);
const xzLevel = ref(6);
const progressMeg = ref("");

const disabled = ref(false);

async function chooseOutFile() {
  const path = await save({
    filters: [
      { name: "encrypted-file-access", extensions: ["encrypted", "enc"] },
      { name: "All Files", extensions: ["*"] },
    ],
  });
  if (path !== null) {
    outFile.value = path;
  }
}

async function chooseSrcFile() {
  const path = await open({
    directory: false,
    multiple: false,
  });
  if (path !== null) {
    srcFile.value = path as string;
  }
}

function create() {
  if (outFile.value === null) {
    progressMeg.value = "Please choose an output file path";
    return;
  }
  progressMeg.value = "In Progress...";
  disabled.value = true;
  invoke("create", {
    outFile: outFile.value,
    srcFile: srcFile.value,
    ext: ext.value,
    noCompress: !compress.value,
    xzLevel: xzLevel.value,
  })
    .then(() => {
      progressMeg.value = "File created successfully";
    })
    .catch((error) => {
      progressMeg.value = error;
    })
    .finally(() => (disabled.value = false));
}
</script>

<template>
  <div>
    <nav>
      <button @click="emit('goto-menu')" :disabled="disabled">
        &larr; Go Back
      </button>
    </nav>
    <h3>Create a Password-Protected File</h3>
  </div>

  <form autocomplete="off" @submit.prevent="create">
    <div class="nowrap">
      Output file:
      <button type="button" @click.prevent="chooseOutFile" :disabled="disabled">
        Choose
      </button>
      {{ outFile ?? "Not chosen" }}
    </div>

    <div class="nowrap">
      Source file (optional):
      <button type="button" @click.prevent="chooseSrcFile" :disabled="disabled">
        Choose
      </button>
      {{ srcFile ?? "Not chosen" }}
    </div>

    <div class="nowrap">
      Extension of source file:
      <input v-model="ext" required :disabled="disabled" />
    </div>

    <div>
      XZ compression:
      {{ compress ? "Enabled" : "Disabled" }}
      <button
        type="button"
        @click.prevent="compress = !compress"
        :disabled="disabled"
      >
        {{ compress ? "Disable" : "Enable" }}
      </button>
    </div>

    <div>
      XZ compression level:
      <input
        type="range"
        v-model="xzLevel"
        min="0"
        max="9"
        :disabled="disabled"
      />
      {{ xzLevel }}
    </div>

    <div class="submit-container">
      <button type="submit" :disabled="disabled">Create</button>
      <span class="errorMsg nowrap">{{ progressMeg }}</span>
    </div>
  </form>
</template>

<style scoped>
h3 {
  text-align: center;
  margin: 0.1em;
}

.nowrap {
  text-wrap: nowrap;
  overflow: hidden;
}

.errorMsg {
  color: red;
  margin-left: 0.5em;
  font-style: italic;
}
</style>
