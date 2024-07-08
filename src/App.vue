<script setup lang="ts">
import { ref } from "vue";
import { open } from "@tauri-apps/api/dialog";
import { documentDir } from "@tauri-apps/api/path";
import { invoke } from "@tauri-apps/api/tauri";

const visible = ref(false);
const checked = ref(false);
const file = ref("");

let tags = [];
const active = ref([])

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
    visible.value = false;
    let tags_to_submit = [];
    for (let i = 0; i < tags.length; i++) {
        if (active.value[i]) {
            tags_to_submit.push(tags[i]);
        }
    }
    invoke("import", {
        path: file.value,
        tags: tags,
    })
        .then(() => emit("close"))
        // TODO handle error
        .catch((err) => console.error(err));
}

invoke("get_tags")
    .then((loaded) => {
        tags = loaded;
        active.value = [];
        for (let i = 0; i < tags.length; i++) {
            active.value.push(false);
        }
    })
    .catch((err) => console.error(err));
</script>

<template>
    <div class="flex justify-center">
        <Button label="Import" @click="visible = true" />

        <Dialog v-model:visible="visible" modal header="Import File" :style="{ width: '60rem' }">
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
    </div>
</template>
