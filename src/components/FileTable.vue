<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/shell";
import { resolveResource } from "@tauri-apps/api/path";

const files = ref([]);
const selectedFile = ref();
var base_path = null;

const onRowClick = async (event) => {
    if (base_path == null) {
        invoke("get_path")
            .then((path) => { base_path = path; onRowClick(event) })
            .catch((err) => console.error(err));
    }
    else {
        const path = base_path + "/" + event.data.file.path + "/" + event.data.file.name;
        console.log(path)
        await open(path)
    }
};

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
    <DataTable class="p-datatable" v-model:selection="selectedFile" selectionMode="multiple" :value="files"
        @row-dblclick="onRowClick" :metaKeySelection="true" removableSort>
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

<style scoped>
.p-datatable {
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
}
</style>
