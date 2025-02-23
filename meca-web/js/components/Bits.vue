<template>
    <div class="bits">
        <div
         v-for="(bit, index) in bitsArray"
         :key="index"
         class="bit-square"
         :class="{ on: bit }"
         ></div>
    </div>
</template>

<script>
export default {
    props: {
        value: {
            type: Number,
            required: true
        },
        bits: {
            type: Number,
            required: true,
        }
    },
    computed: {
        bitsArray() {
            return Array.from({ length: this.bits }, (_, i) => {
                return (this.value & (1 << ((this.bits - 1) - i))) !== 0;
            });
        }
    }
};
</script>

<style scoped>
.bits {
    display: flex;
    gap: 2px;
}
.bit-square {
    width: 5px;
    height: 5px;
    border: 1px solid black;
    background-color: white;
}
.bit-square.on {
    background-color: black;
}
</style>

