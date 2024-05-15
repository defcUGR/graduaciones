<script setup lang="ts">
import { useColorMode } from "@vueuse/core";
import { IconSun, IconMoon } from "@tabler/icons-vue";
import { Toaster } from "@/components/ui/sonner";
import { useEventStore } from "@/stores/eventStore";

const mode = useColorMode();
const event = useEventStore();
const router = useRouter();

watch(event.$state, () => {
  if (event.id) router.push("/event");
});

setTimeout(() => {
  event.id = "Test";
}, 1000);
</script>

<template>
  <Toaster />
  <div class="flex min-h-screen w-full flex-col">
    <header
      class="sticky top-0 flex h-16 items-center justify-between gap-4 border-b bg-background px-4 md:px-6"
    >
      <h1 class="text-xl font-bold">
        Graduaciones{{ event.id ? " - " + event.data : "" }}
      </h1>
      <DropdownMenu>
        <DropdownMenuTrigger as-child>
          <Button variant="outline">
            <IconSun
              class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
            />
            <IconMoon
              class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
            />
            <span class="sr-only">Cambiar tema</span>
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          <DropdownMenuItem @click="mode = 'light'"> Claro </DropdownMenuItem>
          <DropdownMenuItem @click="mode = 'dark'"> Oscuro </DropdownMenuItem>
          <DropdownMenuItem @click="mode = 'auto'"> Sistema </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </header>

    <main class="flex flex-1 flex-col gap-4 p-4 md:gap-8 md:p-8">
      <RouterView />
    </main>
  </div>
</template>
