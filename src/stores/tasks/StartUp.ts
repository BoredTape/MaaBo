interface StartUpTaskParams {
  enable: boolean // 是否启用本任务，可选，默认为 true
  client_type: string // 客户端版本，可选，默认为空
  // 选项："Official" | "Bilibili" | "txwy" | "YoStarEN" | "YoStarJP" | "YoStarKR"
  start_game_enabled: boolean // 是否自动启动客户端，可选，默认不启动
  account_name: string // 切换账号，可选，默认不切换
  // 仅支持切换至已登录的账号，使用登录名进行查找，保证输入内容在所有已登录账号唯一即可
  // 官服：123****4567，可输入 123****4567、4567、123、3****4567
  // B服：张三，可输入 张三、张、三
}

interface StartUpTask {
  name: string
  type: string
  params: StartUpTaskParams
}

export type { StartUpTask, StartUpTaskParams }
