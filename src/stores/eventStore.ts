import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api";

export const useEventStore = defineStore("eventStore", () => {
  const id = ref<string | null>(null);

  const data = computed(() =>
    id.value ? null : invoke("get_event_data", { id: id.value })
  );

  return { id, data };
});
