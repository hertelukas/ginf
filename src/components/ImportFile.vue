<script setup lang="ts">
import { ref, watch, computed } from "vue";
import { open } from "@tauri-apps/api/dialog";
import { documentDir } from "@tauri-apps/api/path";
import { invoke } from "@tauri-apps/api/tauri";

const model = defineModel<{ visible: boolean }>();
const file = ref("");
const newTag = ref("");

const tags = ref([]);
const active = ref([])

const emit = defineEmits(['import'])

const invalidNewTag = computed(() => {
    return newTag.value === "" || tags.value.includes(newTag.value);
})

const invalidImport = computed(() => {
    return file.value === "";
})

async function openFilePicker() {
    const selected = await open({
        multiple: false,
        defaultPath: await documentDir(),
    });
    if (selected && selected.length > 0) {
        file.value = selected;
    }
}

function submit() {
    model.value.visible = false;
    let tags_to_submit = [];
    for (let i = 0; i < tags.value.length; i++) {
        if (active.value[i]) {
            tags_to_submit.push(tags.value[i]);
        }
    }
    invoke("import", {
        path: file.value,
        tags: tags_to_submit,
    })
        .then(() => {
            console.log("Imported");
            emit('import')
        })
        // TODO handle error
        .catch((err) => console.error(err));
}

function insertTag() {
    if (invalidNewTag.value) {
        return;
    }
    invoke("insert_tag", {
        tag: newTag.value,
    })
        .then(() => {
            tags.value.push(newTag.value);
            active.value.push(true);
            newTag.value = "";
        })
        .catch((err) => console.error(err));
}

// Reset state on open
watch(model.value, (value) => {
    if (value.visible) {
        file.value = ""
        invoke("get_tags")
            .then((loaded) => {
                tags.value = loaded;
                active.value = [];
                tags.value.forEach((_) => active.value.push(false));
            })
            .catch((err) => console.error(err));
    }
});
</script>

<template>
    <Dialog v-model:visible="model.visible" modal header="Import File" :style="{ width: '60rem' }" :draggable="false">
        <div class="flex flex-column gap-2">
            <Button icon="pi pi-file" class="w-3" @click="openFilePicker" label="File" />
            <small>{{ file }}</small>
        </div>

        <h3>Tags</h3>
        <div class="flex flex-row flex-wrap gap-3">
            <div v-for="(tag, index) in tags" :key="index">
                <ToggleButton v-model="active[index]" :onLabel="tag" :offLabel="tag" />
            </div>
            <InputText class="w-6rem" type="text" v-model="newTag" @keydown.enter="insertTag" />
            <Button icon="pi pi-plus" :disabled="invalidNewTag" @click="insertTag" />
        </div>

        <Button icon="pi pi-file-import" class="w-3 mt-3" @click="submit" :disabled="invalidImport" label="Import" />
    </Dialog>

</template>
