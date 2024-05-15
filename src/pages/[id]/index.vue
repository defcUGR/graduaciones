<template>
  <div class="grid gap-4 md:gap-8 sm:grid-cols-2">
    <Card>
      <CardHeader>
        <CardTitle>Generar y enviar entradas</CardTitle>
        <CardDescription>
          Genera los códigos, PDFs y similares de los asistentes y envíalos por
          correo.
        </CardDescription>
      </CardHeader>
      <CardFooter>
        <Button
          ><RouterLink :to="route.fullPath + '/send'"
            >Acceder</RouterLink
          ></Button
        >
      </CardFooter>
    </Card>
    <Card>
      <CardHeader>
        <CardTitle>Escanear entradas</CardTitle>
        <CardDescription>
          Accede a la pantalla de escáner y conecta el lector para empezar a
          registrar el acceso
        </CardDescription>
      </CardHeader>
      <CardFooter>
        <Button as-child>
          <RouterLink :to="route.fullPath + '/scan'">Comenzar</RouterLink>
        </Button>
      </CardFooter>
    </Card>
  </div>
</template>

<script setup lang="ts">
const route = useRoute();
</script>
