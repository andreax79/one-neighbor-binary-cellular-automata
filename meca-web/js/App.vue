<template>
    <div class="window" ref="window">
        <div class="window-titlebar" ref="titlebar">
            <button aria-label="Toggle" class="button-toggle" @click="toggle">&#9645;</button>
            <h1 class="title">Settings</h1>
        </div>
        <div class="window-content" ref="content">
            <div>
                <label for="rule">Rule</label>
                <input v-model="rule" type="number" name="rule" step="1" min="0" max="15" />
                <rule v-model="rule" :bits="4" />
            </div>
            <div>
                <label for="steps">Steps</label>
                <input v-model="steps" type="number" name="steps" step="1" min="1" />
            </div>
            <div>
                <label for="boundaries">Boundaries</label>
                <select v-model="boundaries" name="boundaries">
                    <option v-for="(value, index) in boundaries_values" :key="index" :value="value">
                      {{ value }}
                    </option>
                </select>
                <label for="size" class="sub">size:</label>
                <input v-model="size" type="number" name="size" step="1" min="0" />
            </div>
            <div>
                <label for="initial_state">Initial State</label>
                <select v-model="initial_state" name="initial_state">
                    <option v-for="(value, index) in initial_state_values" :key="index" :value="value">
                      {{ value }}
                    </option>
                </select>
                <label for="custom_initial_state" class="sub">pattern:</label>
                <input :disabled="initial_state != 'Custom'" v-model="custom_initial_state" type="text" name="custom_initial_state" />
            </div>
            <div>
                <label for="update_pattern">Update Pattern</label>
                <select v-model="update_pattern" name="update_pattern">
                    <option v-for="(value, index) in update_pattern_values" :key="index" :value="value">
                      {{ value }}
                    </option>
                </select>
                <label for="update_pattern_number" class="sub">n:</label>
                <input :disabled="!update_pattern.endsWith('<n>')" v-model="update_pattern_number" type="number" name="update_pattern_number" />
            </div>
            <div>
                <label for="alpha">Alpha</label>
                <input v-model="alpha" type="number" name="alpha" value="0" step="0.001" min="0" max="1">
            </div>


            <div>
                <label for="color_scheme">Color Scheme</label>
                <select v-model="color_scheme" name="color_scheme">
                    <option v-for="(value, index) in color_scheme_values" :key="index" :value="value">
                      {{ value }}
                    </option>
                </select>
            </div>
            <button @click="render" class="button-primary" id="render">render</button>
        </div>
    </div>
</template>

<script>
import { defineComponent } from 'vue';
import { draw, get_initial_state } from '../pkg/meca_web';
import Rule from './components/Rule.vue'

export default defineComponent({
    components: {
        'rule': Rule,
    },
    data() {
        return get_initial_state();
    },
    methods: {
        render() {
            const canvas = document.getElementById('canvas');
            const ctx = canvas.getContext('2d');
            console.log('rendering');
            const stats = draw(ctx, this, 800, 600);
            console.log(stats);
        },
        toggle() {
            console.log('toggling');
            const content = this.$refs.content;
            content.style.display = content.style.display == 'none' ? 'block' : 'none';
        },
        dragStart(e) {
            e = e || window.event;
            e.preventDefault();
            this.x = e.clientX;
            this.y = e.clientY;
            document.onmouseup = this.dragStop;
            document.onmousemove = this.dragElement;
        },
        dragElement(e) {
            e = e || window.event;
            e.preventDefault();
            const dx = this.x - e.clientX;
            const dy = this.y - e.clientY;
            const win = this.$refs.window;
            win.style.top = (win.offsetTop - dy) + "px";
            win.style.left = (win.offsetLeft - dx) + "px";
            this.x = e.clientX;
            this.y = e.clientY;
        },
        dragStop() {
            document.onmouseup = null;
            document.onmousemove = null;
        }
    },
    mounted() {
        console.log('mounted');
        this.$refs.titlebar.onmousedown = this.dragStart;
        const width = this.$refs.content.getBoundingClientRect().width.toFixed(0);
        this.$refs.titlebar.style.width = width + 'px';
    }
})
</script>
