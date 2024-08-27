<script setup lang="ts">
import { invoke, os } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";
import { ref } from "vue";

const emit = defineEmits<{ "goto-menu": [] }>();

const file = ref<string | null>(null);
const password = ref("");
const executable = ref<string | null>(null);
const progressMeg = ref("");

const disabled = ref(false);

async function chooseFile() {
  const path = await open({
    directory: false,
    multiple: false,
    filters: [
      { name: "encrypted-file-access", extensions: ["encrypted", "enc"] },
      { name: "All Files", extensions: ["*"] },
    ],
  });
  if (path !== null) {
    file.value = path as string;
  }
}

async function chooseExecutable() {
  const filters = [];
  if ((await os.type()) === "Windows_NT") {
    filters.push({
      name: "Executables",
      extensions: ["exe", "com", "bat", "cmd"],
    });
  }
  filters.push({ name: "All Files", extensions: ["*"] });
  const path = await open({
    directory: false,
    multiple: false,
    filters,
  });
  if (path !== null) {
    executable.value = path as string;
  }
}

function openFile() {
  if (file.value === null) {
    progressMeg.value = "Please choose a file";
    return;
  }
  if (password.value === "") {
    progressMeg.value = "Please enter a password";
    return;
  }

  progressMeg.value = "In Progress...";
  disabled.value = true;

  const openArgs = {
    args: {
      file: file.value,
      executable: executable.value,
    },
    password: password.value,
  };
  invoke("open", openArgs)
    .then(() => {
      progressMeg.value = "File opened and saved successfully";
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
    <h3 class="heading">Open a Password-Protected File</h3>
  </div>

  <form @submit.prevent="openFile">
    <div>
      File:
      <button type="button" @click.prevent="chooseFile" :disabled="disabled">
        Choose
      </button>
      {{ file ?? "Not chosen" }}
    </div>

    <div>
      Password:
      <input
        type="password"
        v-model="password"
        required
        autocomplete="current-password"
        :disabled="disabled"
      />
    </div>

    <div class="nowrap">
      Program to open the file (optional):
      <button
        type="button"
        @click.prevent="chooseExecutable"
        :disabled="disabled"
      >
        Choose
      </button>
      {{ executable ?? "Not chosen" }}
    </div>

    <div class="submit-container nowrap">
      <button type="submit" :disabled="disabled">Open file</button>
      <div class="errorMsg">
        <span>{{ progressMeg }}</span>
      </div>
    </div>
  </form>
</template>

<style scoped>
@import "../assets/top-level-pages.css";
</style>
