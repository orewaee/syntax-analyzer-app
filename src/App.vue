<script setup lang="ts">
import {Ref, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {Success, Error} from "./types";

// input
const textarea: Ref<string> = ref("");
const textareaRef = ref<HTMLTextAreaElement | null>(null);

// output
const success: Ref<Success | null> = ref(null);
const error: Ref<Error | null> = ref(null);

function clearOutput() {
    showSemantics.value = false;

    success.value = null;
    error.value = null;
}

function onTextareaInput() {
    clearOutput();
}

const showSemantics: Ref<boolean> = ref(false);

function focusTextarea(index: number) {
    const textarea = textareaRef.value;
    if (textarea == null) return;

    textarea.focus();
    textarea.setSelectionRange(index, index);
}

async function analyze() {
    try {
        const result: Success = JSON.parse(await invoke("analyze", { chain: textarea.value }));
        console.log(result);

        success.value = result;
    } catch (exception) {
        const result: Error = JSON.parse(exception as string);
        console.log(result);

        focusTextarea(result.index + 1);

        error.value = result;
    }
}

function semantics() {
    showSemantics.value = true;
}

function clear() {
    textarea.value = "";
    clearOutput();
}

function selectPreset(preset: string) {
    textarea.value = preset;
    clearOutput();
}

const presets: string[] = [
    "FOR I := 10 TO 20 BY 2 DO;",
    "FOR ABC1D [I, 25, J, 75] := -100 TO -120 BY -5 DO;",
    "FOR A3 := 1 TO 200 DO;"
];
</script>

<template>
<main class="container">
    <div class="presets">
        <button v-for="(preset, i) in presets" :key="i" class="preset" @click="() => selectPreset(preset)">Preset {{i + 1}}</button>
    </div>

    <div class="input">
        <h1 class="title">Input</h1>
        <textarea
            ref="textareaRef"
            class="textarea"
            v-model="textarea"
            @input="onTextareaInput"
            placeholder="Enter a chain..."
        />
    </div>

    <div class="output">
        <h1 class="title">Output</h1>
        <div class="textarea" v-if="success == null && error == null" v-html="'<span class=\'dimmed\'>output</span>'" />
        <div class="textarea" v-else-if="success != null" v-html="success.message.html" />
        <div class="textarea foo" v-else-if="error != null" v-html="error.message.html" />

        <div class="textarea" v-html="(success != null) && showSemantics ? success.semantics : '<span class=\'dimmed\'>semantics</span>'" />
    </div>

    <div class="buttons">
        <button
            :class="'button' + (textarea.length == 0 ? ' inactive' : '')"
            :disabled="textarea.length == 0"
            @click="analyze">
            Analyze
        </button>

        <button
            :class="'button' + (success == null || error != null ? ' inactive' : '')"
            :disabled="success == null || error != null"
            @click="semantics">
            Semantics
        </button>

        <button class="button" @click="clear">Clear</button>
    </div>
</main>
</template>

<style lang="scss">
@import url("https://fonts.googleapis.com/css2?family=Inter:wght@100..900&display=swap");
@import url("https://fonts.googleapis.com/css2?family=Inter:wght@100..900&family=JetBrains+Mono:ital,wght@0,100..800;1,100..800&display=swap");

pre {
    color: var(--FG-500);
}

.right {
    color: var(--GREEN);
}

.wrong {
    color: var(--RED);
}

.dimmed {
    color: var(--FG-100);
}

.accent {
    color: var(--BLUE);
}

* {
    box-sizing: border-box;

    margin: 0;
    padding: 0;
}

body {

    --BG-500: #000;
    --BG-400: #0A0B0A;
    --BG-300: #0F100F;
    --BG-200: #131514;
    --BG-100: #272A29;

    --FG-500: #F4F5F5;
    --FG-400: #D5D8D6;
    --FG-300: #B9C0BD;
    --FG-200: #99A39F;
    --FG-100: #6F7B76;

    --ACCENT: #FFF;

    /*
    --BG-500: #FFF;
    --BG-400: #F4F5F5;
    --BG-300: #EFF0F0;
    --BG-200: #EAEBEB;
    --BG-100: #D5D8D6;

    --FG-500: #0A0B0A;
    --FG-400: #272A29;
    --FG-300: #3F4643;
    --FG-200: #5C6662;
    --FG-100: #84908B;

    --ACCENT: #000;
    */

    --GREEN: #16D886;
    --RED: #E74040;
    --BLUE: #4065E7;

    background-color: var(--BG-500);
}

.container {
    display: flex;
    flex-direction: column;
    gap: 24px;

    padding: 24px;

    .title {
        font-size: 24px;
        font-weight: 500;
        line-height: 24px;

        color: var(--FG-500);
    }

    .textarea {
        min-height: calc(48px * 2);
        width: 100%;

        resize: vertical;

        font-family: "JetBrains Mono", monospace;
        font-size: 16px;

        line-height: 24px;

        color: var(--FG-400);

        border: none;
        border-radius: 16px;
        background-color: var(--BG-200);

        outline: none;

        padding: 12px 24px;

        transition: 240ms ease;

        &:focus {
            box-shadow: 0 0 0 2px var(--ACCENT);
        }
    }
}

.input, .output {
    display: flex;
    flex-direction: column;
    gap: 12px;
}

h1, p, button {
    font-family: Inter, sans-serif;
}

.buttons {
    display: flex;
    gap: 12px;
}

.button {
    flex: 1;

    color: var(--BG-500);

    font-size: 16px;
    font-weight: 500;

    line-height: 24px;

    border: none;
    border-radius: 16px;
    background-color: var(--ACCENT);

    outline: none;
    cursor: pointer;

    padding: 12px 24px;

    transition: 240ms ease;

    &:hover {
        opacity: .8;
    }

    &.inactive {
        color: var(--FG-300);

        background-color: var(--BG-300);

        cursor: not-allowed;
    }
}

.presets {
    display: flex;
    gap: 12px;
}

.preset {
    color: var(--FG-300);

    font-size: 16px;
    font-weight: 500;

    line-height: 24px;

    border: none;
    border-radius: 24px;
    background-color: var(--BG-200);

    outline: none;
    cursor: pointer;

    padding: 12px 24px;

    transition: 240ms ease;

    &:hover {
        opacity: .8;
    }
}
</style>
