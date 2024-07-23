<script setup lang="ts">
import { ref } from "vue";
import ImportFile from "./components/ImportFile.vue"
import FileTable from "./components/FileTable.vue"

const model = ref({visible: false})
</script>

<template>
    <div class="flex justify-center">
        <Button label="Import" @click="() => model.visible = true" />
        <ImportFile v-model="model" />
    </div>

    <FileTable/>

</template>
