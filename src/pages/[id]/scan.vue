<script setup lang="ts">
import {
  IconUsers,
  IconSchool,
  IconMail,
  IconUser,
  IconCheck,
  IconX,
} from "@tabler/icons-vue";
import { Search } from "lucide-vue-next";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { PortService } from "@/ports";
import { TicketData } from "@/schema";
import csv from "@/form_answers.csv";

const input = ref("");
const search = ref("");
const scanned = ref<TicketData[]>([]);
const attendants = ref({});
const filteredAttendants = computed(() => {
  return Object.fromEntries(
    Object.entries(attendants.value).filter(
      ([mail, attendant]) =>
        // @ts-ignore
        mail.includes(search.value) || attendant.name.includes(search.value)
    )
  );
});

PortService.fetchPorts();
console.info("_set fetchPorts");

const ports = PortService.usePorts;
console.info("_set ports", ports);
const selectedPort = ref();
console.info("set selectedPort", selectedPort);

const getPort = () => ports.value[selectedPort.value];

const enterScanning = async () => {
  console.info("go scanning");

  console.info("Mounting port", ports.value[selectedPort.value]);
  const installSuccess = await getPort().install(input);
  if (!installSuccess) return;
  getPort().listen(scanned, input);
};

let total = 0;
(() => {
  for (const row of csv as any) {
    // @ts-ignore
    attendants.value[row.mail] = {
      name: row.name,
      invitations: row.invitations,
      present: computed(() => {
        return (
          scanned.value.filter(
            (ticket) => ticket.email === row.mail && ticket.attendant_type === 0
          ).length != 0
        );
      }),
      invitationsUsed: computed(() => {
        return scanned.value.filter(
          (ticket) => ticket.email === row.mail && ticket.attendant_type === 1
        ).length;
      }),
    };
    total += parseInt(row.invitations) + 1;
  }
})();

const progress = computed(() => {
  const present = scanned.value.length;
  const perc = Math.round((present / total) * 100);
  if (perc === 100 && present != total) return 99;
  else return perc;
});
</script>

<template>
  <h1 v-if="!selectedPort" class="text-xl font-bold">
    Selecciona un método de entrada
  </h1>
  <div v-if="!selectedPort" class="flex flex-row flex-wrap gap-4">
    <Button
      variant="outline"
      v-for="(port, index) in ports"
      @click="
        () => {
          selectedPort = index;
          enterScanning();
        }
      "
      >{{ port.info }}</Button
    >
  </div>
  <div v-else class="flex flex-col-reverse md:flex-row">
    <div class="flex flex-1 flex-col gap-4 p-4 md:gap-8 md:p-8">
      <Card class="xl:col-span-2">
        <CardHeader class="flex flex-row items-center justify-between gap-4">
          <div class="grid gap-2">
            <CardTitle>Asistentes</CardTitle>
          </div>
          <div class="relative items-center">
            <Input
              v-model="search"
              id="search"
              type="text"
              placeholder="Buscar..."
              class="pl-10"
            />
            <span
              class="absolute start-0 inset-y-0 flex items-center justify-center px-2"
            >
              <Search class="size-6 text-muted-foreground" />
            </span>
          </div>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>
                  <div class="flex flex-row items-center">
                    <IconSchool class="w-4 h-4 mr-2" /> Graduado
                  </div></TableHead
                >
                <TableHead>
                  <div class="flex flex-row items-center">
                    <IconMail class="w-4 h-4 mr-2" /> Invitados
                  </div></TableHead
                >
                <TableHead>
                  <div class="flex flex-row items-center">
                    <IconUser class="w-4 h-4 mr-2" /> Presente
                  </div>
                </TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow v-for="(attendant, mail, index) in filteredAttendants">
                <TableCell>
                  <div class="font-medium">
                    {{
                      // @ts-ignore
                      attendant.name
                    }}
                  </div>
                  <div class="text-muted-foreground">{{ mail }}</div>
                </TableCell>
                <TableCell class="text-lg">
                  {{
                    // @ts-ignore
                    attendant.invitationsUsed
                  }}
                  /
                  {{
                    // @ts-ignore
                    attendant.invitations
                  }}
                </TableCell>
                <TableCell>
                  <IconCheck
                    v-if="
                      // @ts-ignore
                      attendant.present
                    "
                    class="w-8 h-8 text-green-500"
                  />
                  <IconX v-else class="w-8 h-8 text-red-500" />
                </TableCell>
              </TableRow>
            </TableBody>
          </Table>
        </CardContent>
      </Card>
    </div>
    <div class="flex flex-2 flex-col gap-4 p-4 md:gap-8 md:p-8">
      <Card>
        <CardHeader
          class="flex flex-row items-center justify-between space-y-0 pb-2"
        >
          <CardTitle class="text-sm font-medium"> Estadísticas </CardTitle>
          <IconUsers class="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold mb-4">
            {{ scanned.length }} / {{ total }} personas
          </div>
          <div class="flex flex-row items-center">
            <Progress v-model="progress" class="w-3/5" />
            <p class="ml-4 mr-1">{{ progress }} %</p>
          </div>
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>Asistentes recientes</CardTitle>
        </CardHeader>
        <CardContent class="grid gap-8">
          <p v-if="scanned.length === 0">No hay asistentes recientes.</p>
          <div
            v-else
            v-for="ticket in scanned.slice(-10).reverse()"
            class="flex items-center gap-4"
          >
            <div class="grid gap-1">
              <p class="text-sm font-medium leading-none">
                {{
                  // @ts-ignore
                  attendants[ticket.email].name
                }}
              </p>
              <p class="text-sm text-muted-foreground">{{ ticket.email }}</p>
            </div>
            <div class="ml-auto font-medium">
              {{ ticket.attendant_type ? "Invitado" : "Graduado" }}
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  </div>
</template>
