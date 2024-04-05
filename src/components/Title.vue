<script setup>
const props = defineProps(['generateTiles', 'score', 'bombs'])
const myString = "hello".toUpperCase()
</script>
<template>
    <div class="title">
        <div class="bombs">
            {{ props.bombs ?? "00" }}
        </div>
        <div class="game">
            {{props.score === 'Win' || props.score === 'Lose' ? props.score.toUpperCase():''}}
        </div>
        <div class="game-button">
            <div @click="props.generateTiles">
                &#128515;
            </div>
        </div>
    </div>
</template>
<style scoped>
.title {
    user-select: none;
    padding: 2vh 0;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 2rem;
}

.bombs {
    font-family: 'DSEG';
    color: white;
    text-decoration: underline;
    font-size: 4vw;
    margin-left: -1rem;
}

.title div {
    width: 33%;
    display: flex;
    justify-content: center;
    align-items: center;
}

.game {
    font-family: Verdana, Geneva, Tahoma, sans-serif;
    color: white;
}

.game-button>div {
    display: flex;
    justify-content: center;
    align-items: center;
    background-color: inherit;
    border: 1px solid transparent;
    border-radius: 5px;
    font-size: 4vw;
    padding: 0.5vh 0.5vw;
}

.game-button>div:hover {
    border: 1px solid white;
}
</style>