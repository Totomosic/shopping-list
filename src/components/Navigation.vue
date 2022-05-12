<template>
  <div class="flex">
    <div class="nav-bar" :style="style"></div>
    <div class="open-button" :style="buttonStyle" @click="open = !open">
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
  cursor: pointer;
  border-top-right-radius: 5px;
  border-bottom-right-radius: 5px;
  position: absolute;
  bottom: 0;
  transition: left 0.3s;
}

.svg-icon {
  margin-top: 0.5em;
}
</style>

<script lang="ts">
import { Options, Vue } from "vue-class-component"
import { isMobileDevice } from "@/utils/utils"

import { ArrowLeft, ArrowRight } from "@element-plus/icons-vue"
import { Prop } from "vue-property-decorator"

export const SIDEBAR_WIDTH = "20em"

@Options({
  components: {
    ArrowLeft,
    ArrowRight,
  },
})
export default class NavigationComponent extends Vue {
  @Prop({ default: !isMobileDevice() }) modelValue!: boolean

  public get style(): any {
    return {
      width: this.open ? SIDEBAR_WIDTH : "0",
    }
  }

  public get buttonStyle(): any {
    return {
      left: this.open ? SIDEBAR_WIDTH : "0",
    }
  }

  public get open(): boolean {
    return this.modelValue
  }

  public set open(value: boolean) {
    this.$emit("update:modelValue", value)
  }
}
</script>
