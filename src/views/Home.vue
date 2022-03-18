<template>
  <Base></Base>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component"

import Base from "@/components/Base.vue"

@Options({
  components: {
    Base,
  },
})
export default class Home extends Vue {}
</script>
