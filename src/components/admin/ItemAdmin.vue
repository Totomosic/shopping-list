<template>
  <div>
    <el-table :data="items">
      <el-table-column label="ID" prop="id"></el-table-column>
      <el-table-column label="Name" prop="name"></el-table-column>
      <el-table-column label="Description" prop="description"></el-table-column>
      <!-- <el-table-column label="Actions">
        <template #default="scope">
          <LoadingButton type="danger" :click="() => deleteUser(scope.row.id)">Delete</LoadingButton>
        </template>
      </el-table-column> -->
    </el-table>
    <div class="text-left mt-2">
      <div v-if="!creatingItem">
        <el-button type="primary" :icon="Plus" @click="startCreatingItem">Add Item</el-button>
      </div>
      <div v-else>
        <h3>Create Item</h3>
        <el-form v-model="newItem" label-width="10em">
          <el-form-item label="Name">
            <el-input v-model="newItem.name"></el-input>
          </el-form-item>
          <el-form-item label="Description">
            <el-input v-model="newItem.description"></el-input>
          </el-form-item>
          <el-form-item label="Unit Type">
            <el-select v-model="newItem.default_unit_type">
              <el-option
                v-for="option of unitTypeOptions"
                :key="option.value"
                :value="option.value"
                :label="option.label"
              ></el-option>
            </el-select>
          </el-form-item>
        </el-form>
        <div>
          <LoadingButton :disabled="!itemValid" type="success" :icon="Plus" :click="createItem"
            >Create Item</LoadingButton
          >
          <el-button type="danger" @click="cancelCreatingItem">Cancel</el-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { AdminDataConnection } from "@/utils/admin_connection"
import { IShoppingItem, INewShoppingItem, UnitType, ILabelledValue } from "@/utils/types"
import { Options, Vue } from "vue-class-component"
import { Prop } from "vue-property-decorator"
import LoadingButton from "../modules/LoadingButton.vue"

import { Plus } from "@element-plus/icons-vue"
import { shallowRef } from "vue"
import { formatUnitType } from "@/utils/utils"
import { ElMessage } from "element-plus"

@Options({
  components: {
    LoadingButton,
  },
})
export default class ItemAdmin extends Vue {
  @Prop(AdminDataConnection) connection!: AdminDataConnection

  public Plus = shallowRef(Plus)
  public readonly Message = ElMessage

  public items: IShoppingItem[] = []
  public creatingItem: boolean = false
  public newItem: INewShoppingItem = {
    name: "",
    description: null,
    image_url: null,
    default_unit_type: UnitType.Count,
  }

  public async mounted() {
    await this.loadItems()
  }

  public get unitTypeOptions(): ILabelledValue<UnitType>[] {
    return [UnitType.Count, UnitType.Mass, UnitType.Capacity].map((v) => ({ label: formatUnitType(v), value: v }))
  }

  public get itemValid(): boolean {
    return this.newItem.name.length > 0
  }

  public async loadItems() {
    this.items = await this.connection.getAllItems()
  }

  public startCreatingItem() {
    this.creatingItem = true
  }

  public async createItem() {
    const item = await this.connection.createNewItem(this.newItem)
    if (item) {
      await this.loadItems()
      this.Message.success(`Successfully created item: ${item.name}`)
    } else {
      this.Message.error(`Failed to create item`)
    }
    this.cancelCreatingItem()
  }

  public cancelCreatingItem() {
    this.creatingItem = false
  }
}
</script>
