<script setup lang="ts">
import {
  IconCirclePlus,
  IconFileImport,
  IconArrowLeft,
  IconZoom,
} from "@tabler/icons-vue";
import { h } from "vue";
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import * as z from "zod";
import { toast } from "vue-sonner";
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { cn } from "@/lib//utils";
import { CalendarDate, parseDate } from "@internationalized/date";
import { vAutoAnimate } from "@formkit/auto-animate/vue";

const creatingNewEvent = ref(false);

const formSchema = toTypedSchema(
  z.object({
    name: z
      .string({ message: "Nombre necesario." })
      .min(2, "Debe tener al menos 2 caracteres.")
      .max(50, "Nombre demasiado largo.")
      .refine((v) => v),
    date: z.string({ message: "Fecha necesaria." }).refine((v) => v),
  })
);

const datePlaceholder = ref();

const { handleSubmit, setFieldValue, values } = useForm({
  validationSchema: formSchema,
  initialValues: {},
});

const date = computed({
  get: () => (values.date ? parseDate(values.date) : undefined),
  set: (val) => val,
});

const onSubmit = handleSubmit((values) => {
  console.log(values);
  toast("You submitted the following values:", {
    description: h(
      "pre",
      { class: "mt-2 max-w-[340px] rounded-md bg-slate-950 p-4" },
      h("code", { class: "text-white" }, JSON.stringify(values, null, 2))
    ),
  });
});
</script>

<template>
  <div class="flex flex-1 flex-col gap-4 p-4 md:gap-8 md:p-8">
    <div
      v-if="creatingNewEvent"
      class="m-2 mt-10 flex justify-center items-center h-full"
    >
      <Card class="max-w-sm">
        <CardHeader>
          <CardTitle
            ><Button
              variant="ghost"
              size="icon"
              class="mr-2"
              @click="creatingNewEvent = false"
              ><IconArrowLeft class="w-4 h-4" /></Button
            >Crear evento</CardTitle
          >
          <CardDescription
            >Genera las entradas de un nuevo evento.</CardDescription
          >
        </CardHeader>
        <CardContent>
          <form class="space-y-6" @submit="onSubmit">
            <FormField v-slot="{ componentField }" name="name">
              <FormItem class="max-w-full" v-auto-animate>
                <FormLabel>Nombre</FormLabel>
                <FormDescription> Nombre completo del evento </FormDescription>
                <FormControl>
                  <Input
                    type="text"
                    placeholder="Graduación Matemáticas 2024"
                    v-bind="componentField"
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            </FormField>
            <FormField name="date">
              <FormItem class="flex flex-col max-w-full" v-auto-animate>
                <FormLabel>Fecha</FormLabel>
                <FormDescription> Fecha de inicio del evento. </FormDescription>
                <Popover>
                  <PopoverTrigger as-child>
                    <FormControl>
                      <Button
                        variant="outline"
                        :class="
                          cn(
                            ' ps-3 text-start font-normal',
                            !date && 'text-muted-foreground'
                          )
                        "
                      >
                        <span>{{ date || "Escoge una fecha" }}</span>
                        <CalendarIcon class="ms-auto h-4 w-4 opacity-50" />
                      </Button>
                      <input hidden />
                    </FormControl>
                  </PopoverTrigger>
                  <PopoverContent class="w-auto p-0">
                    <Calendar
                      v-model:placeholder="datePlaceholder"
                      v-model="date"
                      calendar-label="Fecha"
                      initial-focus
                      :min-value="new CalendarDate(1900, 1, 1)"
                      @update:model-value="
                        (v) => {
                          if (v) {
                            setFieldValue('date', v.toString());
                          } else {
                            setFieldValue('date', undefined);
                          }
                        }
                      "
                    />
                  </PopoverContent>
                </Popover>
                <FormMessage />
              </FormItem>
            </FormField>
            <Button type="submit"> Crear </Button>
          </form>
        </CardContent>
      </Card>
    </div>
    <div v-else class="grid gap-4 md:gap-8 sm:grid-cols-2">
      <Card>
        <CardHeader>
          <CardTitle>Nuevos eventos</CardTitle>
          <CardDescription> Crea o importa un evento nuevo. </CardDescription>
        </CardHeader>
        <CardContent>
          <div class="flex flex-col h-full w-fit">
            <Button
              class="m-2"
              variant="outline"
              @click="creatingNewEvent = true"
              ><IconCirclePlus class="w-4 h-4 mr-2" />Crear evento</Button
            >
            <Button variant="outline" class="m-2"
              ><IconFileImport class="w-4 h-4 mr-2" />Importar un evento</Button
            >
          </div>
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>Eventos existentes</CardTitle>
        </CardHeader>
        <CardContent class="grid gap-8">
          <div class="flex items-center gap-4">
            <div class="grid gap-1">
              <p class="text-sm font-medium leading-none">14/05/2023</p>
              <p class="text-sm text-muted-foreground">
                Graduación Matemáticas 2023
              </p>
            </div>
            <div class="ml-auto font-medium">
              <TooltipProvider>
                <Tooltip>
                  <TooltipTrigger as-child>
                    <Button
                      size="icon"
                      variant="outline"
                      @click="creatingNewEvent = false"
                      ><IconZoom class="w-4 h-4" /></Button></TooltipTrigger
                  ><TooltipContent><p>Consultar</p></TooltipContent>
                </Tooltip></TooltipProvider
              >
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  </div>
</template>
