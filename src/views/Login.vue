<template>
  <Base>
    <el-form :label-width="130">
      <el-form-item label="Username">
        <el-input v-model="username" autocomplete="username"></el-input>
      </el-form-item>
      <el-form-item label="Password">
        <el-input v-model="password" type="password" autocomplete="password"></el-input>
      </el-form-item>
    </el-form>
    <div>
      <el-button type="success" @click="login">Login</el-button>
    </div>
  </Base>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component"
import { fetchJson } from "@/utils/fetch"

import Base from "@/components/Base.vue"
import { ElMessage } from "element-plus/lib/components"
import { GenericResponse, ILoginResponse } from "@/utils/types"
import { isSuccessResponse } from "@/utils/utils"

@Options({
  components: {
    Base,
  },
})
export default class Login extends Vue {
  public message = ElMessage

  public username: string = ""
  public password: string = ""

  public async login(): Promise<void> {
    const response = await fetchJson<GenericResponse<ILoginResponse>>("http://localhost:8000/api/v1/core/auth/basic", {
      method: "POST",
      body: {
        username: this.username,
        password: this.password,
      },
    })
    if (isSuccessResponse(response)) {
      this.message.success("Successfully logged in.")
    } else {
      this.message.error("Failed to login.")
    }
  }
}
</script>
