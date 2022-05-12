<template>
  <div class="w-full h-full flex flex-col">
    <div class="top-bar flex">
      <div class="ml-auto mr-4 mt-auto mb-auto">
        <el-popover
          v-model:visible="popoverOpen"
          effect="dark"
          placement="bottom"
          popper-class="avatar-popover-container"
        >
          <template #default>
            <div class="profile-action" @click="logoutOrLogin">{{ signoutButtonText }}</div>
          </template>
          <template #reference>
            <el-avatar class="profile-image" @click="popoverOpen = !popoverOpen">{{ userInitials }}</el-avatar>
          </template>
        </el-popover>
      </div>
    </div>
    <div class="w-full h-full flex">
      <Navigation v-model="navigationOpen"></Navigation>
      <LoginFormComponent></LoginFormComponent>
      <div class="main-content" :style="mainContentStyle">
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

.profile-image {
  cursor: pointer;
}

.profile-action {
  cursor: pointer;
  padding: 0.5em;
}

.profile-action:hover {
  background-color: #222;
}
</style>

<style>
.el-popover.el-popper.avatar-popover-container {
  padding-left: 0;
  padding-right: 0;
  padding-top: 0.1em;
  padding-bottom: 0.1em;
}
</style>

<script lang="ts">
import { Options, Vue } from "vue-class-component"

import Navigation, { SIDEBAR_WIDTH } from "./Navigation.vue"
import LoginFormComponent from "./LoginForm.vue"
import { useStore } from "@/store"
import { getInitialsFromDisplayName, isMobileDevice } from "@/utils/utils"
import { IJwtToken } from "@/utils/types"

@Options({
  components: {
    Navigation,
    LoginFormComponent,
  },
})
export default class BaseComponent extends Vue {
  private store = useStore()

  public navigationOpen: boolean = !isMobileDevice()
  public popoverOpen: boolean = false

  public async mounted() {
    await this.store.wrappers.auth.loadUser()
  }

  public get user(): IJwtToken | null {
    return this.store.wrappers.auth.user
  }

  public get userInitials(): string {
    if (this.user) {
      return getInitialsFromDisplayName(this.user.display_name)
    }
    return "G"
  }

  public get signoutButtonText(): string {
    return this.user ? "Sign Out" : "Sign In"
  }

  public get mainContentStyle(): any {
    return {
      "max-width": this.navigationOpen ? `calc(100vw - ${SIDEBAR_WIDTH})` : "100vw",
    }
  }

  public async logoutOrLogin() {
    if (this.user) {
      this.store.wrappers.auth.logoutAndGoHome(this.$router)
    } else {
      this.store.wrappers.auth.loadUser()
    }
    this.popoverOpen = false
  }
}
</script>
