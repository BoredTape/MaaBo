interface RecruitTaskParams {
  enable: boolean // 是否启用本任务，可选，默认为 true
  refresh: boolean // 是否刷新三星 Tags, 可选，默认 false
  select: number[] // 会去点击标签的 Tag 等级，必选
  confirm: number[] // 会去点击确认的 Tag 等级，必选。若仅公招计算，可设置为空数组
  first_tags: string[] // 首选 Tags，仅在 Tag 等级为 3 时有效。可选，默认为空, 当 Tag 等级为 3 时，会尽可能多地选择这里的 Tags（如果有）, 而且是强制选择，也就是会忽略所有“让 3 星 Tag 不被选择”的设置
  // extra_tags_mode: number // 选择更多的 Tags, 可选, 默认为 0
  // 0 - 默认行为
  // 1 - 选 3 个 Tags, 即使可能冲突
  // 2 - 如果可能, 同时选择更多的高星 Tag 组合, 即使可能冲突
  times: number // 招募多少次，可选，默认 0。若仅公招计算，可设置为 0
  // set_time: boolean // 是否设置招募时限。仅在 times 为 0 时生效，可选，默认 true
  expedite: boolean // 是否使用加急许可，可选，默认 false
  // expedite_times: number // 加急次数，仅在 expedite 为 true 时有效。
  // 可选，默认无限使用（直到 times 达到上限）
  skip_robot: boolean // 是否在识别到小车词条时跳过，可选，默认跳过
  recruitment_time: object // Tag 等级（大于等于 3）和对应的希望招募时限，单位为分钟，默认值都为 540（即 09:00:00）
  //report_to_penguin: boolean  // 是否汇报企鹅数据，可选，默认 false
  //penguin_id: string       // 企鹅数据汇报 id, 可选，默认为空。仅在 report_to_penguin 为 true 时有效
  //report_to_yituliu: boolean  // 是否汇报一图流数据，可选，默认 false
  //yituliu_id: string       // 一图流汇报 id, 可选，默认为空。仅在 report_to_yituliu 为 true 时有效
  // server: string // 服务器，可选，默认 "CN", 会影响上传
  // 选项："CN" | "US" | "JP" | "KR"
}

interface RecruitTask {
  name: string
  type: string
  params: RecruitTaskParams
}

export type { RecruitTask, RecruitTaskParams }
