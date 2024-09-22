interface AwardTaskParams {
  enable: boolean // 是否启用本任务，可选，默认为 true
  award: boolean // 领取每日/每周任务奖励，默认为 true
  mail: boolean // 领取所有邮件奖励，默认为 false
  recruit: boolean // 领取限定池子赠送的每日免费单抽，默认为 false
  orundum: boolean // 领取幸运墙的合成玉奖励，默认为 false
  mining: boolean // 领取限时开采许可的合成玉奖励，默认为 false
  specialaccess: boolean // 领取五周年赠送的月卡奖励，默认为 false
}

interface AwardTask {
  name: string
  type: string
  params: AwardTaskParams
}

const AwardDefault = (): AwardTask => {
  return {
    name: "领取奖励",
    type: "Award",
    params: {
      award: true,
      enable: true,
      mail: false,
      mining: false,
      orundum: false,
      recruit: false,
      specialaccess: false
    }
  }
}

export { AwardDefault }

export type { AwardTask, AwardTaskParams }
