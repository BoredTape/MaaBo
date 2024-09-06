import { invoke } from '@tauri-apps/api/tauri';

interface StageItem {
    value: string;
    label: string;
}

interface StageData {
    stageId: string;
    code: string;
}

interface Payload {
    code: number;
    msg: string;
    data: StageData[];
}

// 标准 / 磨难
const StageExtraSuffix = {
    NORMAL: '-NORMAL',
    TOUGH: '-HARD',
}

const currentStages: StageItem[] = [
    { label: '当前关卡/上次作战', value: 'NotSpecified' },
]

const appendSuffixAndCreateItems = (stage: any, suffix: string) => {
    const suffixLabel = suffix === StageExtraSuffix.NORMAL ? ' (标准)' : ' (磨难)';
    return {
        value: stage.code + suffix,
        label: stage.code + suffixLabel
    };
};

const basicSupportStages: StageItem[] = [
    { label: '1-7', value: '1-7' },
    { label: '剿灭', value: 'Annihilation' },
    { label: '龙门币-6/5', value: 'CE-6' },
    { label: '经验-6/5', value: 'LS-6' },
    { label: '红票-5', value: 'AP-5' },
    { label: '技能-5', value: 'CA-5' },
    { label: '重装/医疗-小', value: 'PR-A-1' },
    { label: '重装/医疗-大', value: 'PR-A-2' },
    { label: '狙击/术士-小', value: 'PR-B-1' },
    { label: '狙击/术士-大', value: 'PR-B-2' },
    { label: '辅助/先锋-小', value: 'PR-C-1' },
    { label: '辅助/先锋-大', value: 'PR-C-2' },
    { label: '特种/近卫-小', value: 'PR-D-1' },
    { label: '特种/近卫-大', value: 'PR-D-2' },
]

function filterSideStoryStages(sideStoryData: Record<string, any>): StageItem[] {
    const stageItems: StageItem[] = [];

    for (const key of Object.keys(sideStoryData)) {
        // 只取出有效的关卡名称
        const parts = key.split('-');
        if (parts.length > 1 && /^\d+$/.test(parts[1])) {
            stageItems.push({
                value: key,
                label: key
            });
        }
    }

    return stageItems;
}

const GetFightStages = async (): Promise<StageItem[]> => {
    try {
        const result = await invoke<Payload>('get_fight_stages');
        const sideStoryData = await invoke<Payload>('get_current_sidestory');

        const filteredStages = result.data
            .filter(stage => stage.stageId.includes('main') || stage.stageId.includes('sub'))
            .flatMap(stage => {
                // 只要关卡号开头为9（如 9-1, 9-2 等）
                if (stage.code.startsWith('9-')) {
                    return [appendSuffixAndCreateItems(stage, StageExtraSuffix.NORMAL)];
                }
                // 处理关卡号开头为10或更高的情况（如 10-1, 11-1, 12-1 等）
                else if (/^1[0-9]-/.test(stage.code)) {
                    return [
                        appendSuffixAndCreateItems(stage, StageExtraSuffix.NORMAL),
                        appendSuffixAndCreateItems(stage, StageExtraSuffix.TOUGH)
                    ];
                }
                return [{ value: stage.code, label: stage.code }];
            });

        const filteredSideStoryStages = filterSideStoryStages(sideStoryData.data);

        // 合并全部可选关卡
        return [...currentStages, ...filteredSideStoryStages, ...basicSupportStages, ...filteredStages];
    } catch (error) {
        console.error('Failed to retrieve fight stages:', error);
        return [];
    }
}

export type { StageItem };

export default GetFightStages;