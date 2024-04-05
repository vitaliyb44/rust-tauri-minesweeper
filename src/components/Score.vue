<script setup>
const props = defineProps(['score', 'bombs'])
</script>
<template>
    <div class="score">{{ props.bombs }}{{ props.score === 'Win' ? '99' : props.score === 'Lose' ? '00' : '' }}</div>
</template>
<style scoped>
.score {
    font-family: "DSEG";
    font-size: 5rem;
}
</style>