<template>
  <el-dialog
    :model-value="visible"
    :close-on-click-modal="false"
    :close-on-press-escape="false"
    :show-close="false"
    width="30%"
    title="Login"
    @close="closeForm"
  >
    <div>
      <!-- <h3 class="text-left text-2xl font-bold mb-4">Login</h3> -->
      <el-form :label-width="100">
        <el-form-item label="Username">
          <el-input v-model="username"></el-input>
        </el-form-item>
        <el-form-item label="Password">
          <el-input v-model="password" type="password"></el-input>
        </el-form-item>
      </el-form>
      <div v-if="!!error" class="error-text p-4">{{ error }}</div>
      <div>
        <LoadingButton type="primary" :disabled="!canLogin" :click="login">Login</LoadingButton>
        <el-button type="danger" @click="closeForm">Cancel</el-button>
      </div>
    </div>
  </el-dialog>
</template>

<style scoped>
.error-text {
  color: red;
}
</style>

<script lang="ts">
import { useStore } from "@/store"
import { fetchJson } from "@/utils/fetch"
import { saveAccessToken, saveRefreshToken } from "@/utils/jwt"
import { GenericResponse, ILoginResponse } from "@/utils/types"
import { API_BASE, isSuccessResponse } from "@/utils/utils"
import { Options, Vue } from "vue-class-component"

import LoadingButton from "./modules/LoadingButton.vue"

@Options({
  components: {
    LoadingButton,
  },
})
export default class LoginFormComponent extends Vue {
  private store = useStore()

  public username: string = ""
  public password: string = ""
  public error: string = ""

  public get visible(): boolean {
    return this.store.wrappers.auth.loginFormVisible
  }

  public get canLogin(): boolean {
    return this.username.length > 0 && this.password.length > 0
  }

  public closeForm() {
    this.store.wrappers.auth.notifyLoginComplete(false)
  }

  public async login() {
    const response = await fetchJson<GenericResponse<ILoginResponse>>(`${API_BASE}/core/auth/basic`, {
      method: "POST",
      body: {
        username: this.username,
        password: this.password,
      },
    })
    if (isSuccessResponse(response)) {
      saveRefreshToken(response.data.refresh_token)
      saveAccessToken(response.data.access_token)
      this.store.wrappers.auth.notifyLoginComplete(true)
    } else {
      this.error = response?.error ?? "Unable to login"
    }
  }
}
</script>
