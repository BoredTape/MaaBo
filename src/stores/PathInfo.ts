import { GetPath } from '@/apis/Path'
import { defineStore } from 'pinia'
import { ref } from 'vue'

interface Path {
    maabo_dir: string
    maa_config_dir: string
    maa_data_dir: string
    maa_state_dir: string
    maa_cache_dir: string
}


interface Payload {
    code: number
    msg: string
    ts: number
    data: Path
}


export const PathInfoStore = defineStore('PathInfo', () => {
    const path = ref<Path>({} as Path)

    const Load = async () => {
        const res: Payload = await GetPath()
        path.value = res.data
    }
    return { path, Load }
})