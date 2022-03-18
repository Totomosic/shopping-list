<template>
  <div class="w-full h-full flex flex-col">
    <div class="top-bar"></div>
    <div class="w-full h-full flex">
      <Navigation></Navigation>
      <div class="main-content">
        <slot></slot>
      </div>
    </div>
  </div>
</template>

<style scoped>
.top-bar {
  background-color: #222;
  width: 100%;
  height: 5em;
}

.main-content {
  width: 100%;
  height: 100%;
}
</style>

<script lang="ts">
import { Options, Vue } from "vue-class-component"

import Navigation from "./Navigation.vue"

@Options({
  components: {
    Navigation,
  },
})
export default class BaseComponent extends Vue {}
</script>
