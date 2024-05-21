<script lang="ts" setup>

import { onMounted, ref } from "vue";
import { WebContext, GameCore } from "wasm";

const ctx = new WebContext()
const core = new GameCore()
const canvas = ref<HTMLCanvasElement>();

let time = 0
onMounted(()=>{
  ctx.mount(canvas.value!)
  time = Date.now()
  let tickMs = 1000 / 60 //run 30 each sec
  setInterval(update,tickMs);
  render()
})

function update(bool: boolean) {  
  let now = Date.now();
  try { core.update(ctx,now - time) }
  catch (e) { console.log("runtime log:",e) }
  time = now
}
function render() {
  core.render(ctx);
  requestAnimationFrame(render)
}
</script>

<template>
<canvas :class="$style.mainCanvas" ref="canvas" />
</template>

<style lang="scss" module>
.mainCanvas {
  width: 100vw;
  height: 100vh;
}
</style>