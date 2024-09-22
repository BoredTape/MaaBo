<template>
  <div class="box">
    <div class="left-box">
      <TaskList />
    </div>
    <div class="right-box">
      <TaskInfo />
    </div>
    <StartUpSetting />
    <FightSetting />
    <InfrastSetting />
    <MallSetting />
    <AwardSetting />
    <RecruitSetting />
    <ReclamationSetting />
    <RoguelikeSetting />
    <BeforeScriptSetting />
    <AfterScriptSetting />
  </div>
</template>

<script setup lang="ts"></script>

<style lang="scss" scoped>
.box {
  padding-bottom: 0;
  display: flex;
  flex-flow: row wrap;
  justify-content: center;
  align-items: flex-start;
  height: calc(100vh - 126px);
}
.left-box {
  padding-left: 10px;
  padding-right: 10px;
  width: 331px;
  min-height: 388px;
}

.right-box {
  width: 311px;
  min-height: 388px;
}
</style>
