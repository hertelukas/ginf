<script setup lang="ts">
import { ref } from "vue";
import ImportFile from "./components/ImportFile.vue"

const model = ref({visible: false})
</script>

<template>
    <div class="flex justify-center">
        <Button label="Import" @click="() => model.visible = true" />
        <ImportFile v-model="model" />
    </div>
</template>
