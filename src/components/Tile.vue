<script setup>
import { computed, ref } from '@vue/reactivity';
const colorDictiionary = {
    1: '#50BFE6',
    2: '#66FF66',
    3: '#FF355E',
    4: '#C32AFF',
    5: '#FF6037',
    6: '#08E8DE',
    7: '#FFFF66',
    8: '#AAF0D1'
}

const props = defineProps(['row', 'col', 'tile', 'tileInteraction'])

const classObject = computed(() => (
    {
        revealed: props.tile.revealed,
        flagged: props.tile.flagged,
        hidden: !props.tile.revealed && !props.tile.flagged,
        bomb: props.tile.revealed && props.tile.bomb === 'IsBomb'
    }
))
const styleObject = computed(() => ({
    color: colorDictiionary[props.tile.bomb['Bombs']] ?? 'black',
}))

function tileInteracted(interaction) {
    props.tileInteraction(interaction, [props.col, props.row])
}
</script>
<template>
    <div :key="`tile_${props.row}-${props.col}`" @click.middle.prevent="tileInteracted('middle')"
        @contextmenu.prevent="tileInteracted('right')" @click="tileInteracted('left')" class="tile" :class="classObject"
        :style="props.tile.revealed && props.tile.bomb !== 'IsBomb' ? styleObject : ''">
        <template v-if="props.tile.revealed">
            <template v-if="props.tile.bomb === 'IsBomb'">
                <i class="fa-solid fa-bomb"></i>
            </template>
            <template v-else-if="(props.tile.bomb['Bombs'] !== 0)">
                {{props.tile.bomb['Bombs']}}
            </template>
            <template v-else>
                {{''}}
            </template>
        </template>
        <template v-if="props.tile.flagged">
            <i class="fa-solid fa-flag"></i>
        </template>
    </div>
</template>
<style scoped>
.tile {
    font-family: 'Gill Sans', 'Gill Sans MT', Calibri, 'Trebuchet MS', sans-serif;
    font-weight: bold;
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: 2vw;
    border: 1px solid var(--davys-grey);
    border-radius: 5px;
    background-color: black;
    user-select: none;
    aspect-ratio: 1/1;
}

.flagged {
    color: crimson;
}

.revealed {
    background-color: #454545;
}

.bomb {
    color: red;
}

.hidden {

    border: outset #454545;
}

.hidden:hover {
    border-style: inset;
}
</style>