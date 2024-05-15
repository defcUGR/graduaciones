<template>
  <Card>
    <CardHeader>
      <CardTitle>Configuración del envío</CardTitle>
      <CardDescription
        >Configura qué y cómo quieres enviarlo a los asistentes</CardDescription
      >
    </CardHeader>
    <CardContent>
      <form @submit="onSubmit" class="grid grid-cols-1 sm:grid-cols-2 gap-3">
        <FormField name="formats">
          <FormItem v-auto-animate>
            <div class="mb-4">
              <FormLabel class="text-base"> Formatos a enviar </FormLabel>
            </div>

            <FormField
              v-for="format in formats"
              v-slot="{ value, handleChange }"
              :key="format.id"
              type="checkbox"
              :value="format.id"
              :unchecked-value="false"
              name="formats"
            >
              <FormItem class="flex flex-row items-start space-x-3 space-y-0">
                <FormControl>
                  <Checkbox
                    :checked="value.includes(format.id)"
                    @update:checked="handleChange"
                  />
                </FormControl>
                <FormLabel class="font-normal">
                  {{ format.label }}
                </FormLabel>
              </FormItem>
            </FormField>
            <FormMessage />
          </FormItem>
        </FormField>

        <FormField name="methods">
          <FormItem v-auto-animate>
            <div class="mb-4">
              <FormLabel class="text-base"> Métodos de envío </FormLabel>
            </div>

            <FormField
              v-for="method in methods"
              v-slot="{ value, handleChange }"
              :key="method.id"
              type="checkbox"
              :value="method.id"
              :unchecked-value="false"
              name="methods"
            >
              <FormItem class="flex flex-row items-start space-x-3 space-y-0">
                <FormControl>
                  <Checkbox
                    :checked="value.includes(method.id)"
                    @update:checked="handleChange"
                  />
                </FormControl>
                <FormLabel class="font-normal">
                  {{ method.label }}
                </FormLabel>
              </FormItem>
            </FormField>
            <FormMessage />
          </FormItem>
        </FormField>

        <div class="flex justify-start mt-4 col-span-full">
          <Button type="submit"> Seleccionar archivo de datos </Button>
        </div>
      </form>
    </CardContent>
  </Card>
</template>

<script setup lang="ts">
import { z } from "zod";
import { h } from "vue";
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import { toast } from "vue-sonner";
import { open } from "@tauri-apps/api/dialog";

import { vAutoAnimate } from "@formkit/auto-animate";
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";

const formats = [
  {
    id: "qr_png",
    label: "QR como imagen",
  },
  {
    id: "ticket_pdf",
    label: "Entrada en PDF",
  },
  {
    id: "pkpass",
    label: "Archivo PKPass para iOS Wallet",
  },
] as const;
const methods = [
  {
    id: "email",
    label: "Correo Electrónico",
  },
] as const;

const formSchema = toTypedSchema(
  z.object({
    formats: z.array(z.string()).refine((value) => value.some((item) => item), {
      message: "Selecciona al menos un formato.",
    }),
    methods: z.array(z.string()).refine((value) => value.some((item) => item), {
      message: "Selecciona al menos un método.",
    }),
  })
);

const { handleSubmit } = useForm({
  validationSchema: formSchema,
  initialValues: {
    formats: ["ticket_pdf"],
    methods: ["email"],
  },
});

const onSubmit = handleSubmit(async (values) => {
  const path = await open({
    directory: false,
    filters: [{ name: "CSV", extensions: ["csv"] }],
  });
  toast(
    "You submitted the following path: " +
      path +
      "; and values: " +
      JSON.stringify(values)
  );
});
</script>
