<template>
    <div class="bit-editor">
        <label v-for="(bit, index) in bitsArray" :key="index" class="bit-checkbox">
            <bits :value="(bits - 1) - index" :bits="Math.log2(bits)" />
            <div :class="{ on: bit, checkbox: true }" @click="toggleBit(index)"></div>
        </label>
    </div>
</template>

<script>
import Bits from './Bits.vue'

export default {
    components: {
        'bits': Bits,
    },
    props: {
        modelValue: {
            type: Number,
            required: true
        },
        bits: {
            type: Number,
            required: true
        }
    },
    computed: {
        bitMask() {
            return (1 << this.bits) - 1;
        },
        bitsArray() {
            return Array.from({ length: this.bits }, (_, i) => {
                return (this.modelValue & (1 << ((this.bits - 1) - i))) !== 0;
            });
        }
    },
    methods: {
        toggleBit(index) {
            let newValue = this.modelValue ^ (1 << ((this.bits - 1) - index));
            newValue &= this.bitMask;
            this.$emit("update:modelValue", newValue);
        }
    }
};
</script>

<style scoped>
.bit-editor {
    display: flex;
    gap: 4px;
}
.bit-checkbox {
    display: flex;
    flex-direction: column;
    align-items: center;
    font-size: 12px;
    border: 1px solid black;
    padding: 2px;
}
.bit-checkbox .checkbox {
    margin-top: 2px;
    width: 5px;
    height: 5px;
    border: 1px solid black;
    background-color: white;
}
.bit-checkbox .checkbox.on {
    background-color: black;
}
.bit-checkbox input[type=checkbox] {
    width: 1em;
}
</style>

