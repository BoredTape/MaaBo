
interface VideoRecognitionParams {
    enable: boolean
    filename: string
}

interface VideoRecognition {
    name: string
    type: string
    params: VideoRecognitionParams
}

export type { VideoRecognition, VideoRecognitionParams }