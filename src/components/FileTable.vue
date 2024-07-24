<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/shell";
import { resolveResource } from "@tauri-apps/api/path";
import { FilterMatchMode, FilterOperator } from "@primevue/core/api";
import ImportFile from "./ImportFile.vue"

const model = ref({ visible: false })
const files = ref([]);
const filters = ref();
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
        await open(path)
    }
};

const initFilters = () => {
    filters.value = {
        global: { value: null, matchMode: FilterMatchMode.CONTAINS },
        'file.name': { operator: FilterOperator.AND, constraints: [{ value: null, matchMode: FilterMatchMode.STARTS_WITH }] },
    };
};

const clearFilter = () => {
    initFilters();
}

const initFiles = () => {
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
}

initFilters();
initFiles();
</script>

<template>
    <ImportFile v-model="model" @import="initFiles()" />
    <DataTable class="p-datatable" v-model:selection="selectedFile" selectionMode="multiple" :value="files"
        @row-dblclick="onRowClick" :metaKeySelection="true" removableSort v-model:filters="filters" filterDisplay="menu"
        :globalFilterFields="['file.name']">
        <template #header>
            <div class="flex justify-content-between">
                <div class="flex gap-2">
                    <Button label="Import" icon="pi pi-file-import" @click="() => model.visible = true" outlined />
                    <Button type="button" icon="pi pi-filter-slash" label="Clear" outlined @click="clearFilter()" />
                </div>
                <IconField>
                    <InputIcon>
                        <i class="pi pi-search" />
                    </InputIcon>
                    <InputText v-model="filters['global'].value" placeholder="Keyword Search" />
                </IconField>
            </div>
        </template>
        <Column field="file.name" header="File" sortable>
            <template #filter="{ filterModel }">
                <InputText v-model="filterModel.value" type="text" placeholder="Filter by name" />
            </template>
        </Column>
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
