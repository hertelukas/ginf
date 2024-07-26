<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/shell";
import { resolveResource } from "@tauri-apps/api/path";
import { FilterMatchMode, FilterOperator, FilterService } from "@primevue/core/api";
import ImportFile from "./ImportFile.vue"

const model = ref({ visible: false })
const files = ref([]);
const filters = ref();
const selectedFile = ref();
const unique_tags = ref([]);
var base_path = null;

const TAGS_FILTER = ref("TAG_FILTER");

FilterService.register(TAGS_FILTER.value, (value, filter): boolean => {
    return filter.every((filter_tag) => {
        return value.some((tag) => {
            return tag.tag.toLowerCase() === filter_tag.toLowerCase();
        });
    });
});

const tagsContainMatchModes = [{ label: "Contains", value: TAGS_FILTER.value }]

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
        tags: { value: null, matchMode: TAGS_FILTER.value }
    };
};

const clearFilter = () => {
    initFilters();
}

const initFiles = () => {
    invoke("get_files")
        .then((loaded) => {
            unique_tags.value = [];
            files.value = loaded.map(item => {
                item[1].forEach((tag) => {
                    if (!unique_tags.value.includes(tag.tag)) {
                        unique_tags.value.push(tag.tag);
                    }
                });
                return {
                    file: item[0],
                    tags: item[1],
                }
            });
            unique_tags.value.sort();
        })
        .catch((err) => console.error(err));
}

initFilters();
initFiles();
</script>

<template>
    <ImportFile v-model="model" @import="initFiles()" />
    <DataTable v-model:selection="selectedFile" selectionMode="multiple" :value="files" @row-dblclick="onRowClick"
        :metaKeySelection="true" removableSort v-model:filters="filters" filterDisplay="menu"
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
        <Column field="tags" header="Tags" :filter-match-mode-options="tagsContainMatchModes">
            <template #body="{ data }">
                <div class="flex flex-wrap gap-2">
                    <Tag severity="success" v-for="tag in data.tags">{{ tag.tag }}</Tag>
                </div>
            </template>
            <template #filter="{ filterModel, filterCallback }">
                <MultiSelect v-model="filterModel.value" filter :options="unique_tags" placeholder="Filter by tag"
                    @change="filterCallback()">
                    <template #option="slotProps">
                        <Tag :value="slotProps.option" />
                    </template>
                </MultiSelect>
                <!-- <InputText v-model="filterModel.value" type="text" placeholder="Filter by tag" /> -->
            </template>
        </Column>
    </DataTable>
</template>
