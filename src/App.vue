<script setup lang="ts">
import { useColorMode } from "@vueuse/core";
import { IconSun, IconMoon, IconSlash } from "@tabler/icons-vue";
import { Toaster } from "@/components/ui/sonner";
import { useEventStore } from "@/stores/eventStore";
import { sift } from "radash";
import { CalendarDate } from "@internationalized/date";
import { isCuid } from "@paralleldrive/cuid2";

const mode = useColorMode();
const event = useEventStore();
const router = useRouter();
const route = useRoute();

const events = [
  {
    id: "xpw12i396k",
    name: "Graduación Matemáticas 2024",
    date: new CalendarDate(2024, 5, 14),
  },
  {
    id: "tezdzyx66t",
    name: "Graduación Física 2024",
    date: new CalendarDate(2024, 5, 14 + 7),
  },
];

const crumbs = computed(() => sift(route.fullPath.split("/")));

const crumbName = computed(() => (crumb: string) => {
  if (crumb === "scan") return "Escanear";
  if (crumb === "send") return "Enviar";
  else return events.find((e) => e.id === crumb)?.name || "ERROR";
});

watch(route, (val) => {
  if (val.fullPath === "/") event.id = null;
  else if (isCuid(val.fullPath.replace("/", "")))
    event.id = val.fullPath.split("/")[1];

  console.info("evid", event.id, val.fullPath);
});

setTimeout(() => {
  // event.id = "Test";
}, 1000);
</script>

<template>
  <Toaster />
  <div class="flex min-h-screen w-full flex-col">
    <header
      class="sticky top-0 flex h-16 items-center gap-4 border-b bg-background px-4 md:px-6"
    >
      <h1 class="text-xl font-bold">
        Graduaciones<span v-if="event.id">{{ " - " + event.data }}</span>
      </h1>

      <Breadcrumb class="flex-grow">
        <BreadcrumbList>
          <BreadcrumbItem>
            <BreadcrumbLink as-child>
              <RouterLink to="/">Home</RouterLink>
            </BreadcrumbLink>
          </BreadcrumbItem>
          <template v-for="(crumb, idx) in crumbs">
            <BreadcrumbSeparator>
              <IconSlash />
            </BreadcrumbSeparator>
            <BreadcrumbItem>
              <BreadcrumbLink as-child>
                <RouterLink :to="'/' + crumbs.slice(0, idx + 1).join('/')">{{
                  crumbName(crumb)
                }}</RouterLink>
              </BreadcrumbLink>
            </BreadcrumbItem>
          </template>
        </BreadcrumbList>
      </Breadcrumb>

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
