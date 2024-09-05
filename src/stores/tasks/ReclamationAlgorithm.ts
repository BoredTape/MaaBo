interface ReclamationAlgorithmTaskParams {
  enable: boolean
  theme: number // 主题，可选项。默认为 1
  // 0 - *沙中之火*
  // 1 - *沙洲遗闻*
  mode: number // 模式，可选项。默认为 0
  // 0 - 刷分与建造点，进入战斗直接退出
  // 1 - 沙中之火：刷赤金，联络员买水后基地锻造；
  //     沙洲遗闻：自动制造物品并读档刷货币
  product: string // 自动制造的物品，可选项，默认为荧光棒
  // 建议填写子串
}

interface ReclamationAlgorithmTask {
  name: string
  type: string
  params: ReclamationAlgorithmTaskParams
}

export type { ReclamationAlgorithmTask, ReclamationAlgorithmTaskParams }
