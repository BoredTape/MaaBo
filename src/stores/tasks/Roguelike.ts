interface RoguelikeTaskParams {
  enable: boolean // 是否启用本任务，可选，默认为 true
  theme: string // 肉鸽名，可选，默认 "Phantom"
  // Phantom - 傀影与猩红血钻
  // Mizuki  - 水月与深蓝之树
  // Sami    - 探索者的银凇止境
  mode: number // 模式，可选项。默认 0
  // 0 - 刷蜡烛，尽可能稳定地打更多层数
  // 1 - 刷源石锭，第一层投资完就退出
  // 2 - 【即将弃用】两者兼顾，投资过后再退出，没有投资就继续往后打
  // 3 - 开发中...
  // 4 - 刷开局，到达第三层后直接退出
  // 5 - 刷坍缩范式，mode为5时有效，通过漏怪、在不期而遇节点选择特定选项等方式加快坍缩值积累，
  //     若第一个检测到的坍缩范式在expected_collapsal_paradigms列表中则停止任务，否则重开
  starts_count: number // 开始探索 次数，可选，默认 INT_MAX。达到后自动停止任务

  investment_enabled: boolean // 是否投资源石锭，默认开
  investments_count: number
  // 投资源石锭 次数，可选，默认 INT_MAX。达到后自动停止任务
  stop_when_investment_full: boolean
  // 投资满了自动停止任务，可选，默认 false

  squad: string // 开局分队，可选，例如 "突击战术分队" 等，默认 "指挥分队"
  roles: string // 开局职业组，可选，例如 "先手必胜" 等，默认 "取长补短"
  core_char: string // 开局干员名，可选，仅支持单个干员**中文名**，无论区服。默认识别练度自动选择

  start_with_elite_two: boolean // 是否在刷开局模式下凹开局干员精二直升，可选，默认 false
  only_start_with_elite_two: boolean // 是否只凹开局干员精二直升且不进行作战，可选，默认 false，start_with_elite_two为true时有效

  use_support: boolean // 开局干员是否为助战干员，可选，默认 false
  use_nonfriend_support: boolean // 是否可以是非好友助战干员，可选，默认 false，use_support为true时有效

  refresh_trader_with_dice: boolean // 是否用骰子刷新商店购买特殊商品，目前支持水月肉鸽的指路鳞，可选，默认 false

  use_foldartal: boolean // 是否使用密文板，mode为5时默认 false，否则默认 true
  check_collapsal_paradigms: boolean // 是否检测获取的坍缩范式，mode为5时默认 true，否则默认 false
  double_check_collapsal_paradigms: boolean // 是否执行坍缩范式检测防漏措施，check_collapsal_paradigms为true时有效，
  // mode为5时默认 true，否则默认 false
  expected_collapsal_paradigms: string[] // 需要刷的坍缩范式，mode为5时有效
  // 默认 ["目空一些, "睁眼瞎", "图像损坏", "一抹黑"]
}

interface RoguelikeTask {
  name: string
  type: string
  params: RoguelikeTaskParams
}

export type { RoguelikeTask, RoguelikeTaskParams }
