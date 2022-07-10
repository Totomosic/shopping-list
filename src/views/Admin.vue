<template>
  <Base>
    <div class="admin-container">
      <el-tabs v-model="currentTab">
        <el-tab-pane name="user" label="Users">
          <UserAdmin :connection="connection"></UserAdmin>
        </el-tab-pane>
        <el-tab-pane name="item" label="Items">
          <ItemAdmin :connection="connection"></ItemAdmin>
        </el-tab-pane>
      </el-tabs>
    </div>
  </Base>
</template>

<style scoped>
.admin-container {
  background-color: #eee;
  border-radius: 5px;
  margin: 2em;
  padding: 2em;
}
</style>

<script lang="ts">
import { Options, Vue } from "vue-class-component"

import Base from "@/components/Base.vue"
import UserAdmin from "@/components/admin/UserAdmin.vue"
import ItemAdmin from "@/components/admin/ItemAdmin.vue"

import { useStore } from "@/store"
import { AdminDataConnection } from "@/utils/admin_connection"

@Options({
  components: {
    Base,
    UserAdmin,
    ItemAdmin,
  },
})
export default class AdminView extends Vue {
  private store = useStore()

  public connection = new AdminDataConnection(this.store)
  public currentTab: string = "user"

  public async mounted() {
    await this.store.wrappers.auth.loadAdminUserOrGoHome(this.$router)
  }
}
</script>
