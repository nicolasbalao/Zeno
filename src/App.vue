<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

type Application = {
  name: string;
};

const apps = ref<Application[]>([]);

async function getApps() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  apps.value = await invoke("get_applications_names");
}

const onInput = async (event: Event) => {
  const input = (event.target as HTMLInputElement).value;
  if (input.length > 0) {
    // Search for applications based on the input
    apps.value = await invoke("search_application", {
      name: input,
      applicationNames: apps.value,
    });
  } else {
    // If input is empty, fetch all applications
    getApps();
  }
};

const onClickApplication = async (appName: string) => {
  await invoke("launch_application_cmd", {
    name: appName,
  });
};

onMounted(() => {
  // Fetch applications when the component is mounted
  getApps();
});
</script>

<template>
  <main class="container">
    <div>
      <input
        type="text"
        placeholder="Search applications..."
        @input="onInput"
      />
    </div>
    <ul>
      <li v-for="app in apps" @click="onClickApplication(app.name)">
        {{ app.name }}
      </li>
    </ul>
  </main>
</template>
