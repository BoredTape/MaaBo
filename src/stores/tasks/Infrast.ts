interface InfrastTaskParams {
  enable: boolean // 是否启用本任务，可选，默认为 true
  mode: number // 换班工作模式，可选，默认 0
  // 0 - 默认换班模式，单设施最优解
  // 10000 - 自定义换班模式，读取用户配置，可参考 protocol/base-scheduling-schema.md

  facility: string[] // 要换班的设施（有序），必选。不支持运行中设置
  // 设施名，"Mfg" | "Trade" | "Power" | "Control" | "Reception" | "Office" | "Dorm"

  drones: string // 无人机用途，可选项，默认 _NotUse
  // mode==10000 时该字段无效（会被忽略）
  // "_NotUse"、"Money"、"SyntheticJade"、"CombatRecord"、"PureGold"、"OriginStone"、"Chip"
  threshold: number // 工作心情阈值，可选，取值范围 [0, 1.0]，默认 0.3
  // mode==10000 时该字段仅针对 "autofill" 有效
  replenish: boolean // 贸易站“源石碎片”是否自动补货，可选，默认 false

  dorm_notstationed_enabled: boolean // 是否启用宿舍“未进驻”选项，可选，默认 false
  dorm_trust_enabled: boolean // 是否将宿舍剩余位置填入信赖未满干员，可选，默认 false

  /* 以下参数仅在 mode=10000 时生效，否则会被忽略 */
  filename: string // 自定义配置路径，必选。不支持运行中设置
  plan_index: number // 使用配置中的方案序号，必选。不支持运行中设置
}

interface InfrastTask {
  name: string
  type: string
  params: InfrastTaskParams
}

const InfrastDefault = (): InfrastTask => {
  return {
    name: "基建换班",
    type: "Infrast",
    params: {
      dorm_notstationed_enabled: true,
      dorm_trust_enabled: true,
      drones: "Money",
      enable: true,
      facility: [
        "Trade",
        "Reception",
        "Mfg",
        "Control",
        "Power",
        "Office",
        "Dorm"
      ],
      filename: "",
      mode: 0,
      plan_index: 0,
      replenish: false,
      threshold: 0.3
    }
  }
}

export { InfrastDefault }
export type { InfrastTask, InfrastTaskParams }
