<script setup>
import Tile from './Tile.vue';

let props = defineProps(['board', 'tileInteraction'])

</script>
<template>
    <div class="board">
        <template v-if="(props.board.length !== 0)">
            <template v-for="(row, rowIndex) in props.board">
                <Tile :tileInteraction="tileInteraction" :row="rowIndex" :col="colIndex" :tile="tile"
                    :key="`${rowIndex}_row,${colIndex}_col`" v-for="(tile, colIndex) in row">
                </Tile>
            </template>
        </template>
    </div>
</template>
<style scoped>
.board {
    height: 100%;
    margin: 0 auto;
    width: 100%;
    display: grid;
    grid: repeat(16, 1fr) / repeat(30, 1fr);
    border-top: 2px dashed lightgray;
    border-bottom: 2px dashed lightgray;
}
</style>
