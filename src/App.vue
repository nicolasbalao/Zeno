<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

type Application = {
  name: string;
};

const apps = ref<Application[]>([]);
const searchInput = ref("");
const selectedIndex = ref(0);

async function getApps() {
  apps.value = await invoke("get_applications_names");
}

const onInput = async (event: Event) => {
  const input = (event.target as HTMLInputElement).value;
  searchInput.value = input;
  selectedIndex.value = 0; // Reset selection when searching

  if (input.length > 0) {
    apps.value = await invoke("search_application", {
      name: input,
      applicationNames: apps.value,
    });
  } else {
    getApps();
  }
};

const onClickApplication = async (appName: string) => {
  await invoke("launch_application_cmd", {
    name: appName,
  });
  searchInput.value = ""; // Clear search input after launching
};

// Keyboard navigation
const handleKeyDown = (event: KeyboardEvent) => {
  switch (event.key) {
    case "ArrowDown":
      event.preventDefault();
      selectedIndex.value = Math.min(
        selectedIndex.value + 1,
        apps.value.length - 1
      );
      break;
    case "ArrowUp":
      event.preventDefault();
      selectedIndex.value = Math.max(selectedIndex.value - 1, 0);
      break;
    case "Enter":
      event.preventDefault();
      if (apps.value[selectedIndex.value]) {
        onClickApplication(apps.value[selectedIndex.value].name);
      }
      break;
    case "Escape":
      searchInput.value = "";
      getApps();
      break;
  }
};

onMounted(() => {
  getApps();
});
</script>

<template>
  <main class="launcher">
    <div class="search-container">
      <div class="test"></div>
      <div class="search-icon">
        <svg
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <circle cx="11" cy="11" r="8" />
          <path d="m21 21-4.35-4.35" />
        </svg>
      </div>
      <input
        type="text"
        placeholder="Type to search applications..."
        class="search-input"
        :value="searchInput"
        @input="onInput"
        @keydown="handleKeyDown"
        autofocus
      />
    </div>

    <div class="results-container" v-if="apps.length > 0">
      <div class="results-header">
        <span class="results-count">{{ apps.length }} applications</span>
      </div>
      <ul class="results-list">
        <li
          v-for="(app, index) in apps"
          :key="app.name"
          :class="['result-item', { selected: index === selectedIndex }]"
          @click="onClickApplication(app.name)"
          @mouseenter="selectedIndex = index"
        >
          <div class="app-icon">
            <svg
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
              <circle cx="9" cy="9" r="2" />
              <path d="m21 15-3.086-3.086a2 2 0 0 0-1.414-.586H13" />
            </svg>
          </div>
          <span class="app-name">{{ app.name }}</span>
        </li>
      </ul>
    </div>

    <div v-else-if="searchInput.length > 0" class="no-results">
      <span>No applications found</span>
    </div>
  </main>
</template>

<style>
* {
  box-sizing: border-box;
}

body {
  margin: 0;
  padding: 0;
  width: 100vw;
  height: 100vh;
}

#app {
  width: 100%;
  height: 100%;
  background-color: #f3f4f6;
}
</style>

<style scoped>
.launcher {
  width: 100%;
  height: 100%;
  background: #ffffff;
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1),
    0 10px 10px -5px rgba(0, 0, 0, 0.04);
  border: 1px solid #e5e7eb;
  overflow: hidden;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "Roboto",
    sans-serif;
}

.search-container {
  position: relative;
  padding: 16px;
  border-bottom: 1px solid #f3f4f6;
}

.search-icon {
  position: absolute;
  left: 24px;
  top: 50%;
  transform: translateY(-50%);
  color: #9ca3af;
  pointer-events: none;
}

.search-input {
  width: 100%;
  padding: 12px 16px 12px 44px;
  border: none;
  outline: none;
  font-size: 16px;
  border-radius: 8px;
  background: #f9fafb;
  transition: background-color 0.2s ease;
  box-sizing: border-box;
}

.search-input:focus {
  background: #ffffff;
  box-shadow: 0 0 0 2px #3b82f6;
}

.search-input::placeholder {
  color: #9ca3af;
}

.results-container {
  height: 100%;
  overflow-y: auto;
}

.results-header {
  padding: 8px 16px;
  background: #f9fafb;
  border-bottom: 1px solid #f3f4f6;
}

.results-count {
  font-size: 12px;
  color: #6b7280;
  font-weight: 500;
}

.results-list {
  list-style: none;
  margin: 0;
  padding: 0;
}

.result-item {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  cursor: pointer;
  transition: all 0.15s ease;
  border-bottom: 1px solid #f3f4f6;
}

.result-item:last-child {
  border-bottom: none;
}

.result-item:hover,
.result-item.selected {
  background: #f3f4f6;
}

.result-item.selected {
  background: #eff6ff;
  border-left: 3px solid #3b82f6;
}

.app-icon {
  margin-right: 12px;
  color: #6b7280;
  flex-shrink: 0;
}

.app-name {
  font-size: 14px;
  color: #374151;
  font-weight: 500;
}

.no-results {
  padding: 32px 16px;
  text-align: center;
  color: #9ca3af;
  font-size: 14px;
}

/* Scrollbar styling */
.results-container::-webkit-scrollbar {
  width: 6px;
}

.results-container::-webkit-scrollbar-track {
  background: #f3f4f6;
}

.results-container::-webkit-scrollbar-thumb {
  background: #d1d5db;
  border-radius: 3px;
}

.results-container::-webkit-scrollbar-thumb:hover {
  background: #9ca3af;
}

/* Dark mode support */
@media (prefers-color-scheme: dark) {
  .launcher {
    background: #1f2937;
    border-color: #374151;
  }

  .search-container {
    border-bottom-color: #374151;
  }

  .search-input {
    background: #374151;
    color: #f9fafb;
  }

  .search-input:focus {
    background: #4b5563;
  }

  .results-header {
    background: #374151;
    border-bottom-color: #4b5563;
  }

  .results-count {
    color: #9ca3af;
  }

  .result-item {
    border-bottom-color: #374151;
  }

  .result-item:hover {
    background: #374151;
  }

  .result-item.selected {
    background: #1e40af;
  }

  .app-name {
    color: #f9fafb;
  }

  .no-results {
    color: #6b7280;
  }
}
</style>
