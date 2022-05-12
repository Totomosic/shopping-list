<template>
  <div>
    <el-table :data="users">
      <el-table-column label="ID" prop="id"></el-table-column>
      <el-table-column label="Display Name" prop="display_name"></el-table-column>
      <el-table-column label="Is Admin" prop="is_admin"></el-table-column>
    </el-table>
  </div>
</template>

<script lang="ts">
import { AdminDataConnection } from "@/utils/admin_connection"
import { IUser } from "@/utils/types"
import { Vue } from "vue-class-component"
import { Prop } from "vue-property-decorator"

export default class UserAdmin extends Vue {
  @Prop(AdminDataConnection) connection!: AdminDataConnection

  public users: IUser[] = []

  public async mounted() {
    await this.loadUsers()
  }

  public async loadUsers() {
    this.users = await this.connection.getAllUsers()
  }
}
</script>
