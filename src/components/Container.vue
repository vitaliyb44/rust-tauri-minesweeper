<script setup>
import { invoke } from "@tauri-apps/api"
import { ref } from 'vue';
import Board from './Board.vue';
import Title from './Title.vue';

const game = ref({
    board: [],
    state: 'Starting',
    bombs: "00"
})
console.log(game.value.board)
async function generateTiles() {
    await invoke("generate_board").then(data => game.value = data)
}
async function tileInteraction(interaction, tile) {
    if (game.value.state !== 'Lose') await invoke('tile_interaction', { interaction, tile }).then(data => game.value = data)
}
</script>

<template>
    <div class="board-container">
        <Title :score="game.state" :bombs="game.bombs" :generateTiles="generateTiles"></Title>
        <Board :board="game.board" :tileInteraction="tileInteraction"></Board>
    </div>
</template>

<style scoped>
.board-container {
    overflow: hidden;
    min-width: 100%;
    margin: auto;
    min-height: 100%;
    display: flex;
    flex-direction: column;
}
</style>