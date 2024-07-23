<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
const files = ref([]);

invoke("get_files")
    .then((loaded) => {
        files.value = loaded.map(item => {
            return {
                file: item[0],
                tags: item[1],
            }
        });
    })
    .catch((err) => console.error(err));

</script>

<template>
    <DataTable class="border-round" :value="files">
        <Column field="file.name" header="File" sortable> </Column>
        <Column header="Tags">
            <template #body="{ data }">
                <div class="flex gap-2">
                    <Tag severity="success" v-for="tag in data.tags">{{ tag.tag }}</Tag>
                </div>
            </template>
        </Column>
    </DataTable>
</template>
