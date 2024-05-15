<script setup lang="ts">
import { PortService } from "../ports";

PortService.fetchPorts();
console.info("_set fetchPorts");

const ports = PortService.usePorts;
const selectedPort = ref();
</script>

<template>asdfasdfas</template>
