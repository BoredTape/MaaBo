
interface OperBoxParams {
    enable: boolean
}

interface OperBox {
    name: string
    type: string
    params: OperBoxParams
}

export type { OperBox, OperBoxParams }