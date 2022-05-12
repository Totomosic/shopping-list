<template>
  <el-button :disabled="disabled || loading" :size="size" :type="type" :icon="currentIcon" @click="handleClick">
    <slot></slot>
  </el-button>
</template>

<script lang="ts">
import { Vue } from "vue-class-component"
import { Prop } from "vue-property-decorator"

export default class LoadingButton extends Vue {
  @Prop({ default: "default" }) size!: string
  @Prop({ default: "primary" }) type!: string
  @Prop({ default: false }) disabled!: boolean
  @Prop({ default: null }) icon!: string | null
  @Prop({ default: null }) click!: (() => Promise<void>) | null

  public loading: boolean = false

  public get currentIcon() {
    if (this.loading) {
      return "el-icon-loading"
    }
    return this.icon
  }

  public async handleClick() {
    if (this.click) {
      this.loading = true
      await this.click()
      this.loading = false
    }
  }
}
</script>
