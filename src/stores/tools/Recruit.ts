interface RecruitmentTime {
    [key: string]: number
}

interface RecruitParams {
    enable: boolean
    refresh: boolean
    select: number[]
    confirm: number[]
    first_tags: string[]
    times: number
    set_time: boolean
    expedite: boolean
    skip_robot: boolean
    recruitment_time: RecruitmentTime
}

interface Recruit {
    name: string
    type: string
    params: RecruitParams
}

export type { Recruit, RecruitParams, RecruitmentTime }