interface MallTaskParams {
  enable: boolean // 是否启用本任务，可选，默认为 true
  shopping: boolean // 是否购物，可选，默认 false。不支持运行中设置
  buy_first: string[] // 优先购买列表，可选。不支持运行中设置,商品名，如 "招聘许可"、"龙门币" 等
  blacklist: string[] // 黑名单列表，可选。不支持运行中设置,商品名，如 "加急许可"、"家具零件" 等
  force_shopping_if_credit_full: boolean // 是否在信用溢出时无视黑名单，默认为 true
  only_buy_discount: boolean // 是否只购买折扣物品，只作用于第二轮购买，默认为 false
  reserve_max_credit: boolean // 是否在信用点低于300时停止购买，只作用于第二轮购买，默认为 false
}

interface MallTask {
  name: string
  type: string
  params: MallTaskParams
}

export type { MallTask, MallTaskParams }
