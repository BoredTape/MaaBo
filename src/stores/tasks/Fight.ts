interface FightTaskParams {
  enable: boolean // 是否启用本任务，可选，默认为 true
  stage: string // 关卡名，可选，默认为空，识别当前/上次的关卡。不支持运行中设置
  // 支持全部主线关卡，如 "1-7"、"S3-2"等
  // 可在关卡结尾输入Normal/Hard表示需要切换标准与磨难难度
  // 剿灭作战，必须输入 "Annihilation"
  // 当期 SS 活动 后三关，必须输入完整关卡编号
  medicine: number // 最大使用理智药数量，可选，默认 0
  expiring_medicine: number // 最大使用 48 小时内过期理智药数量，可选，默认 0
  stone: number // 最大吃石头数量，可选，默认 0
  times: number // 指定次数，可选，默认无穷大
  series: number // 连战次数，可选，1~6
  drops?: object // 指定掉落数量，可选，默认不指定
  // key - item_id, value 数量. key 可参考 resource/item_index.json 文件
  // 是或的关系

  /* 以上全部是或的关系，即任一达到即停止任务 */

  //report_to_penguin: boolean  // 是否汇报企鹅数据，可选，默认 false
  //penguin_id: string       // 企鹅数据汇报 id, 可选，默认为空。仅在 report_to_penguin 为 true 时有效
  // server: string // 服务器，可选，默认 "CN", 会影响掉落识别及上传
  // 选项："CN" | "US" | "JP" | "KR"
  client_type: string // 客户端版本，可选，默认为空。用于游戏崩溃时重启并连回去继续刷，若为空则不启用该功能
  // 选项："Official" | "Bilibili" | "txwy" | "YoStarEN" | "YoStarJP" | "YoStarKR"
  // DrGrandet: boolean // 节省理智碎石模式，可选，默认 false，仅在可能产生碎石效果时生效。
  // 在碎石确认界面等待，直到当前的 1 点理智恢复完成后再立刻碎石
}

interface FightTask {
  name: string
  type: string
  params: FightTaskParams
}

export type { FightTask, FightTaskParams }
