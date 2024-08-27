<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";
import { ref } from "vue";

const emit = defineEmits<{ "goto-menu": [] }>();

const file = ref<string | null>(null);
const oldPassword = ref("");
const newPassword = ref("");
const compress = ref<boolean | null>(null);
const xzLevel = ref<number | null>(null);
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

async function updatePassword() {
  if (file.value === null) {
    progressMeg.value = "Please choose a file";
    return;
  }
  if (oldPassword.value === "") {
    progressMeg.value = "Please enter the current password";
    return;
  }
  if (newPassword.value === "") {
    progressMeg.value = "Please enter a new password";
    return;
  }
  if (oldPassword.value === newPassword.value) {
    progressMeg.value = "New password is the same as current password";
    return;
  }

  progressMeg.value = "In Progress...";
  disabled.value = true;

  const updatePasswordArgs = {
    args: {
      file: file.value,
      no_compress: null,
      xz_level: null,
    },
    oldPassword: oldPassword.value,
    newPassword: newPassword.value,
  };
  invoke("update_password", updatePasswordArgs)
    .then(() => {
      progressMeg.value = "Password updated successfully";
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
    <h3 class="heading">Update Password</h3>
  </div>

  <form @submit.prevent="updatePassword">
    <div>
      File:
      <button type="button" @click.prevent="chooseFile" :disabled="disabled">
        Choose
      </button>
      {{ file ?? "Not chosen" }}
    </div>

    <div>
      Current password:
      <input
        type="password"
        v-model="oldPassword"
        required
        autocomplete="current-password"
        :disabled="disabled"
      />
    </div>

    <div>
      New password:
      <input
        type="password"
        v-model="newPassword"
        required
        autocomplete="new-password"
        :disabled="disabled"
      />
    </div>

    <div>
      XZ compression:
      <select v-model="compress" :disabled="disabled">
        <option :value="null">Keep original settings</option>
        <option :value="true">Enable</option>
        <option :value="false">Disable</option>
      </select>
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

    <div class="submit-container nowrap">
      <button type="submit" :disabled="disabled">Update password</button>
      <div class="errorMsg">
        <span>{{ progressMeg }}</span>
      </div>
    </div>
  </form>
</template>

<style scoped>
@import "../assets/top-level-pages.css";
</style>
