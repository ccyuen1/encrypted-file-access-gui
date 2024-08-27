<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { open, save } from "@tauri-apps/api/dialog";
import { ref } from "vue";

const emit = defineEmits<{ "goto-menu": [] }>();

const outFile = ref<string | null>(null);
const srcFile = ref<string | null>(null);
const ext = ref("");
const compress = ref(true);
const xzLevel = ref(6);
const progressMeg = ref("");
const password = ref("");
const confirmPassword = ref("");

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
  if (password.value === "") {
    progressMeg.value = "Please enter a password";
    return;
  }
  if (password.value !== confirmPassword.value) {
    progressMeg.value = "Passwords do not match";
    return;
  }

  progressMeg.value = "In Progress...";
  disabled.value = true;

  const createArgs = {
    args: {
      out_file: outFile.value,
      src: srcFile.value,
      extension: ext.value,
      no_compress: !compress.value,
      xz_level: xzLevel.value,
    },
    password: password.value,
  };
  invoke("create", createArgs)
    .then(() => {
      progressMeg.value = "File created successfully";
    })
    .catch((error) => {
      progressMeg.value = error.toString();
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
    <h3 class="heading">Create a Password-Protected File</h3>
  </div>

  <form @submit.prevent="create">
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
      <input
        v-model="ext"
        autocomplete="off"
        placeholder="e.g., txt, docx, pdf"
        :required="!srcFile"
        :disabled="disabled || !!srcFile"
      />
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
        v-model.number="xzLevel"
        min="0"
        max="9"
        autocomplete="off"
        :disabled="disabled || !compress"
      />
      {{ xzLevel }}
    </div>

    <div>
      Create a password:
      <input
        type="password"
        v-model="password"
        required
        autocomplete="new-password"
        :disabled="disabled"
      />
    </div>

    <div>
      Enter the password again:
      <input
        type="password"
        v-model="confirmPassword"
        required
        autocomplete="new-password"
        :disabled="disabled"
      />
    </div>

    <div class="submit-container nowrap">
      <button type="submit" :disabled="disabled">Create</button>
      <div class="errorMsg">
        <span>{{ progressMeg }}</span>
      </div>
    </div>
  </form>
</template>

<style scoped>
@import "../assets/top-level-pages.css";
</style>
