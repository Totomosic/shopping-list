<template>
  <div class="flex">
    <div class="nav-bar" :style="style"></div>
    <div class="open-button" @click="open = !open">
      <div class="text-white">
        <ArrowLeft v-if="open" class="svg-icon"></ArrowLeft>
        <ArrowRight v-else class="svg-icon"></ArrowRight>
      </div>
    </div>
  </div>
</template>

<style scoped>
.nav-bar {
  transition: width 0.3s;
  background-color: #111;
}

.open-button {
  width: 2em;
  height: 3em;
  background-color: #111;
  margin-top: auto;
  cursor: pointer;
  border-top-right-radius: 5px;
  border-bottom-right-radius: 5px;
}

.svg-icon {
  margin-top: 0.5em;
}
</style>

<script lang="ts">
import { Options, Vue } from "vue-class-component"
import { isMobileDevice } from "@/utils/utils"

import { ArrowLeft, ArrowRight } from "@element-plus/icons-vue"

@Options({
  components: {
    ArrowLeft,
    ArrowRight,
  },
})
export default class NavigationComponent extends Vue {
  public open: boolean = !isMobileDevice()

  public get style(): any {
    return {
      width: this.open ? "20em" : "0",
    }
  }
}
</script>
