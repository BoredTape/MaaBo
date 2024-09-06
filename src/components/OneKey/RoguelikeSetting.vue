<template>
  <el-dialog
    v-model="dialogVisible['Roguelike']"
    title="无限肉鸽设置"
    center
    align-center
    style="width: 350px"
    destroy-on-close
    :before-close="saveSetting"
  >
    <el-tabs class="setting-tabs" v-model="activeName">
      <el-tab-pane label="普通设置" name="Normal">
        <el-form
          label-width="152px"
          style="padding-left: 2px; padding-right: 2px; padding-top: 2px"
        >
          <el-form-item label="肉鸽名">
            <el-select
              v-model="params.theme"
              style="width: 150px"
              :disabled="userConfig!.status == 1 && params.enable"
            >
              <el-option label="傀影与猩红血钻" value="Phantom" default />
              <el-option label="水月与深蓝之树" value="Mizuki" />
              <el-option label="探索者的银凇止境" value="Sami" />
              <el-option label="萨卡兹的无终奇语" value="Sarkaz" />
            </el-select>
          </el-form-item>
          <el-form-item label="开始探索次数">
            <el-input-number
              v-model="params.starts_count"
              controls-position="right"
              :disabled="userConfig!.status == 1 && params.enable"
            />
          </el-form-item>

          <el-form-item label="模式">
            <el-select
              v-model="params.mode"
              style="width: 150px"
              @change="modeChange"
              :disabled="userConfig!.status == 1 && params.enable"
            >
              <el-option label="刷蜡烛/通关" :value="0" default />
              <el-option label="刷源石锭" :value="1" />
              <el-option label="刷开局" :value="4" />
              <el-option label="刷坍缩范式" :value="5" />
            </el-select>
          </el-form-item>
        </el-form>

        <el-form
          v-show="mode4Show"
          label-width="152px"
          style="
            border: 1px solid var(--el-border-color);
            padding-left: 2px;
            padding-right: 2px;
            padding-top: 2px;
          "
        >
          <el-form-item label="凹开局干员精二直升">
            <el-switch
              v-model="params.start_with_elite_two"
              :disabled="userConfig!.status == 1 && params.enable"
            />
          </el-form-item>
          <el-form-item label="直升且不进行作战" v-show="params.start_with_elite_two">
            <el-switch
              v-model="params.only_start_with_elite_two"
              :disabled="userConfig!.status == 1 && params.enable"
            />
          </el-form-item>
        </el-form>

        <el-form
          label-width="152px"
          v-show="mode5Show"
          style="
            border: 1px solid var(--el-border-color);
            padding-left: 2px;
            padding-right: 2px;
            padding-top: 2px;
          "
        >
          <el-form-item label="使用密文板">
            <el-switch
              v-model="params.use_foldartal"
              :disabled="userConfig!.status == 1 && params.enable"
            />
          </el-form-item>
          <el-form-item label="检测坍缩范式">
            <el-switch
              v-model="params.check_collapsal_paradigms"
              :disabled="userConfig!.status == 1 && params.enable"
            />
          </el-form-item>
          <el-form-item label="执行坍缩范式防漏措施" v-show="mode5Show">
            <el-switch
              v-model="params.double_check_collapsal_paradigms"
              :disabled="userConfig!.status == 1 && params.enable"
            />
          </el-form-item>
          <el-form-item label="坍缩范式">
            <el-select
              v-model="params.expected_collapsal_paradigms"
              multiple
              :disabled="userConfig!.status == 1 && params.enable"
            >
              <el-option label="目空一些" value="目空一些" />
              <el-option label="睁眼瞎" value="睁眼瞎" />
              <el-option label="图像损坏" value="图像损坏" />
              <el-option label="一抹黑" value="一抹黑" />
            </el-select>
          </el-form-item>
        </el-form>

        <el-form
          label-width="152px"
          style="padding-left: 2px; padding-right: 2px; padding-top: 2px"
        >
          <el-form-item label="开局分队">
            <el-select
              v-model="params.squad"
              style="width: 150px"
              :disabled="userConfig!.status == 1 && params.enable"
            >
              <el-option label="心胜于物分队" value="心胜于物分队" />
              <el-option label="物尽其用分队" value="物尽其用分队" />
              <el-option label="以人为本分队" value="以人为本分队" />
              <el-option label="永恒狩猎分队" value="永恒狩猎分队" />
              <el-option label="生活至上分队" value="生活至上分队" />
              <el-option label="科学主义分队" value="科学主义分队" />
              <el-option label="特训分队" value="特训分队" />
              <el-option label="魂灵护送分队" value="魂灵护送分队" />
              <el-option label="博闻广记分队" value="博闻广记分队" />
              <el-option label="蓝图测绘分队" value="蓝图测绘分队" />
              <el-option label="因地制宜分队" value="因地制宜分队" />
              <el-option label="指挥分队" value="指挥分队" default />
              <el-option label="集群分队" value="集群分队" />
              <el-option label="后勤分队" value="后勤分队" />
              <el-option label="矛头分队" value="矛头分队" />
              <el-option label="突击战术分队" value="突击战术分队" />
              <el-option label="堡垒战术分队" value="堡垒战术分队" />
              <el-option label="远程战术分队" value="远程战术分队" />
              <el-option label="破坏战术分队" value="破坏战术分队" />
              <el-option label="研究分队" value="研究分队" />
              <el-option label="高规格分队" value="高规格分队" />
            </el-select>
          </el-form-item>
          <el-form-item label="开局职业组">
            <el-select
              v-model="params.roles"
              style="width: 150px"
              :disabled="userConfig!.status == 1 && params.enable"
            >
              <el-option label="先手必胜(先锋、狙击、特种)" value="先手必胜" />
              <el-option label="取长补短(近卫、辅助、医疗)" value="取长补短" default />
              <el-option label="稳扎稳打(重装、术师、狙击)" value="稳扎稳打" />
              <el-option label="随心所欲(三张随机)" value="随心所欲" />
            </el-select>
          </el-form-item>
          <el-form-item label="开局干员名">
            <el-input
              v-model="params.core_char"
              style="width: 150px"
              :disabled="userConfig!.status == 1 && params.enable"
            />
          </el-form-item>
        </el-form>
      </el-tab-pane>
      <el-tab-pane label="高级设置" name="Advanced">
        <el-form
          label-width="152px"
          style="padding-left: 2px; padding-right: 2px; padding-top: 2px"
        >
          <el-form-item label="是否投资源石锭">
            <el-switch
              v-model="params.investment_enabled"
              @change="investmentChange"
              :disabled="userConfig!.status == 1 && params.enable"
            />
          </el-form-item>
        </el-form>
        <el-form
          label-width="152px"
          v-show="investmentShow"
          style="
            border: 1px solid var(--el-border-color);
            padding-left: 2px;
            padding-right: 2px;
            padding-top: 2px;
          "
        >
          <el-form-item label="投资源石锭次数">
            <el-input-number
              v-model="params.investments_count"
              controls-position="right"
              :disabled="userConfig!.status == 1 && params.enable"
            />
          </el-form-item>
          <el-form-item label="投资满了自动停止">
            <el-switch
              v-model="params.stop_when_investment_full"
              :disabled="userConfig!.status == 1 && params.enable"
            />
          </el-form-item>
        </el-form>
        <el-form label-width="152px">
          <el-form-item label="开局干员选助战干员">
            <el-switch
              v-model="params.use_support"
              :disabled="userConfig!.status == 1 && params.enable"
            />
          </el-form-item>
          <el-form-item label="选非好友助战干员" v-show="params.use_support">
            <el-switch
              v-model="params.use_nonfriend_support"
              :disabled="userConfig!.status == 1 && params.enable"
            />
          </el-form-item>
          <el-form-item label="用骰子刷新商店购买特殊商品">
            <el-switch
              v-model="params.refresh_trader_with_dice"
              :disabled="userConfig!.status == 1 && params.enable"
            />
          </el-form-item>
        </el-form>
      </el-tab-pane>
    </el-tabs>
    <template #footer></template>
  </el-dialog>
</template>

<script setup lang="ts">
import { type RoguelikeTaskParams } from '@/stores/tasks/Roguelike'
import { UserConfigStore } from '@/stores/UserConfig'
import { ref } from 'vue'
const userConfigStore = UserConfigStore()
const userConfig = ref(userConfigStore.GetConfig(userConfigStore.selectedConfig))
const dialogVisible = ref(userConfigStore.GetSettingDialogObj())
const params = ref<RoguelikeTaskParams>(
  userConfigStore.GetTaskParams('Roguelike') as RoguelikeTaskParams
)
const saveSetting = () => {
  if (userConfig.value!.status === 0) {
    userConfigStore.SaveTask()
  }
  dialogVisible.value['Roguelike'] = false
}

const activeName = ref('Normal')

const investmentShow = ref(params.value.investment_enabled)
const investmentChange = () => {
  investmentShow.value = params.value.investment_enabled
}

const mode5Show = ref(false)
const mode4Show = ref(false)
const modeChange = () => {
  if (params.value.mode === 5) {
    params.value.use_foldartal = false
    params.value.check_collapsal_paradigms = true
    params.value.double_check_collapsal_paradigms = true
    mode5Show.value = true
  } else {
    params.value.use_foldartal = true
    params.value.check_collapsal_paradigms = false
    params.value.double_check_collapsal_paradigms = false
    mode5Show.value = false
  }

  if (params.value.mode === 4) {
    mode4Show.value = true
  } else {
    mode4Show.value = false
  }
}
</script>

<style lang="scss" scoped>
.el-form-item {
  margin-bottom: 2px;
}
</style>
