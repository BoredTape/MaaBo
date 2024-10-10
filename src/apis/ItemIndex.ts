import { invoke } from '@tauri-apps/api/core'

interface FightItem {
    value: string
    label: string
}


interface Item {
    classifyType: string
    description: string
    icon: string
    name: string
    sortId: number
    usage: string
}

interface ItemIndex {
    [key: string]: Item
}

interface Payload {
    code: number
    msg: string
    data: ItemIndex
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
                FightItems.push({ value: val, label: (res as Payload).data[val].name })
            }
        }
    })
    return FightItems
}



const GetItemIndex = async (): Promise<ItemIndex | null> => {
    await invoke('get_item_index').then((res) => {
        return (res as Payload).data
    })
    return null
}

export type { FightItem, Item, ItemIndex }
export { GetItemIndex }
export default GetFightItems