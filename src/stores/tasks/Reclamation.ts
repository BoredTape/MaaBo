interface ReclamationTaskParams {
  enable: boolean
  theme: string // 主题，可选项。默认为 Fire
  // Fire  - *沙中之火*
  // Tales - *沙洲遗闻*
  mode: number // 模式，可选项。默认为 0
  // 0 - 刷分与建造点，进入战斗直接退出
  // 1 - 沙中之火：刷赤金，联络员买水后基地锻造；
  //     沙洲遗闻：自动制造物品并读档刷货币
  tool_to_craft: string // 自动制造的物品，可选项，默认为荧光棒
  // 建议填写子串
  increment_mode: number // 点击类型，可选项。默认为0
  // 0 - 连点
  // 1 - 长按
  num_craft_batches: number // 单次最大制造轮数，可选。默认为 16
}

interface ReclamationTask {
  name: string
  type: string
  params: ReclamationTaskParams
}

export type { ReclamationTask, ReclamationTaskParams }
