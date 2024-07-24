<script setup lang="ts">
import { ref, watch } from "vue";
import { open } from "@tauri-apps/api/dialog";
import { documentDir } from "@tauri-apps/api/path";
import { invoke } from "@tauri-apps/api/tauri";

const model = defineModel<{ visible: boolean }>();
const file = ref("");

const tags = ref([]);
const active = ref([])

const emit = defineEmits(['import'])

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
            <Button class="w-3" @click="openFilePicker">File</Button>
            <small>{{ file }}</small>
        </div>

        <h3>Tags</h3>
        <div class="flex flex-row flex-wrap gap-3">
            <div v-for="(tag, index) in tags" :key="index">
                <ToggleButton v-model="active[index]" :onLabel="tag" :offLabel="tag" />
            </div>
        </div>

        <Button class="w-3 mt-3" @click="submit">Import</Button>
    </Dialog>

</template>
