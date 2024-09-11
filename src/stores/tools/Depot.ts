
interface DepotParams {
    enable: boolean
}

interface Depot {
    name: string
    type: string
    params: DepotParams
}

export type { Depot, DepotParams }