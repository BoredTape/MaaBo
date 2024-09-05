interface Connection {
  preset?: string // MuMuPro
  adb_path: string // 如果你需要的话，你可以覆盖预设的 adb 路径，大多数情况下你不需要这么做
  address: string // emulator-5554
  config?: string
}


interface Resource {
  global_resource: string | null, //"Official" | "Bilibili" | "txwy" | "YoStarEN" | "YoStarJP" | "YoStarKR"
  //     platform_diff_resource:string,
  //     user_resource:boolean
}

interface StaticOptions {
  cpu_ocr?: boolean // false
  gpu_ocr?: number // 1
}

interface InstanceOptions {
  touch_mode: string //"ADB/MiniTouch/MaaTouch/MacPlayTools", for android12 or higher using fucking "MaaTouch".
  deployment_with_pause: boolean // false
  adb_lite_enabled: boolean //false
  kill_adb_on_exit: boolean //false
}

interface MaaCoreConfig {
  connection: Connection
  resource?: Resource
  static_options: StaticOptions
  instance_options: InstanceOptions
}

export type { MaaCoreConfig, Resource }
