<script setup lang="ts">
import { ref } from "vue";
const visible = ref(false);
</script>
<template>
    <div class="card flex justify-center">
        <Button label="Import" @click="visible = true" />
        <Dialog v-model:visible="visible" modal header="Import File" :style="{ width: '50%' }">
        </Dialog>
    </div>
</template>
