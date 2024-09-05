import { invoke } from '@tauri-apps/api/tauri'

interface FightItem {
    value: string
    label: string
}

interface PayloadData {
    [key: string]: { [key: string]: any }
}

interface Payload {
    code: number
    msg: string
    data: PayloadData
}

const GetFightItems = async (): Promise<FightItem[]> => {
    const FightItems: FightItem[] = [
        {
            value: 'NotSpecified',
            label: '不指定',
        }
    ]
    await invoke('get_item_index').then((res) => {
        for (const val in (res as Payload).data) {
            if (!isNaN(Number(val))) {
                FightItems.push({ value: val, label: (res as Payload).data[val]["name"] })
            }
        }
    })
    return FightItems


}

export type { FightItem }

export default GetFightItems