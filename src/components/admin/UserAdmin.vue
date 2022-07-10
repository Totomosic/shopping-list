<template>
  <div>
    <el-table :data="users">
      <el-table-column label="ID" prop="id"></el-table-column>
      <el-table-column label="Display Name" prop="display_name"></el-table-column>
      <el-table-column label="Is Admin" prop="is_admin"></el-table-column>
      <el-table-column label="Actions">
        <template #default="scope">
          <LoadingButton type="danger" :click="() => deleteUser(scope.row.id)">Delete</LoadingButton>
        </template>
      </el-table-column>
    </el-table>
    <div class="text-left mt-2">
      <div v-if="!creatingUser">
        <el-button type="primary" @click="startCreatingUser">Create User</el-button>
      </div>
      <div v-else>
        <h3>Create User</h3>
        <el-form v-model="newUser" label-width="10em">
          <el-form-item label="Display Name">
            <el-input v-model="newUser.display_name"></el-input>
          </el-form-item>
          <el-form-item label="Username">
            <el-input v-model="newUser.username"></el-input>
          </el-form-item>
          <el-form-item label="Password">
            <el-input v-model="newUser.password" type="password"></el-input>
          </el-form-item>
          <el-form-item label="Admin">
            <el-switch v-model="newUser.is_admin"></el-switch>
          </el-form-item>
        </el-form>
        <div>
          <LoadingButton :disabled="!validUser" type="success" :icon="Plus" :click="createUser"
            >Create User</LoadingButton
          >
          <el-button type="danger" @click="cancelCreatingUser">Cancel</el-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { AdminDataConnection } from "@/utils/admin_connection"
import { INewUser, IUser } from "@/utils/types"
import { Options, Vue } from "vue-class-component"
import { Prop } from "vue-property-decorator"
import LoadingButton from "../modules/LoadingButton.vue"

import { Plus } from "@element-plus/icons-vue"
import { shallowRef } from "vue"
import { ElMessage } from "element-plus"

@Options({
  components: {
    LoadingButton,
  },
})
export default class UserAdmin extends Vue {
  @Prop(AdminDataConnection) connection!: AdminDataConnection

  public Plus = shallowRef(Plus)
  public readonly Message = ElMessage

  public users: IUser[] = []
  public creatingUser: boolean = false
  public newUser: INewUser = {
    display_name: "",
    username: "",
    password: "",
    is_admin: false,
  }

  public async mounted() {
    await this.loadUsers()
  }

  public get validUser(): boolean {
    return this.newUser.display_name.length > 0 && this.newUser.username.length > 0 && this.newUser.password.length > 0
  }

  public async loadUsers() {
    this.users = await this.connection.getAllUsers()
  }

  public startCreatingUser() {
    this.creatingUser = true
  }

  public async createUser() {
    const user = await this.connection.createNewUser(this.newUser)
    if (user) {
      await this.loadUsers()
      this.Message.success(`Successfully created user: ${user.display_name}`)
    } else {
      this.Message.error(`Failed to create user`)
    }
    this.cancelCreatingUser()
  }

  public async deleteUser(userId: number) {
    await this.connection.deleteUser(userId)
    await this.loadUsers()
  }

  public cancelCreatingUser() {
    this.creatingUser = false
  }
}
</script>
