<template>
  <Base></Base>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component"

import Base from "@/components/Base.vue"
import { useStore } from "@/store"
import { fetchJsonAuthenticated } from "@/utils/fetch"
import { API_BASE } from "@/utils/utils"

@Options({
  components: {
    Base,
  },
})
export default class HomeView extends Vue {
  private store = useStore()

  public async mounted() {
    const users = await fetchJsonAuthenticated(`${API_BASE}/users`, this.store)
    console.log(users)
  }
}
</script>
