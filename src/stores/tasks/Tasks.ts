import { type AwardTask } from './Award'
import { type FightTask } from './Fight'
import { type InfrastTask } from './Infrast'
import { type MallTask } from './Mall'
import { type ReclamationAlgorithmTask } from './ReclamationAlgorithm'
import { type RecruitTask } from './Recruit'
import { type RoguelikeTask } from './Roguelike'
import { type StartUpTask } from './StartUp'

type Task =
  | AwardTask
  | FightTask
  | InfrastTask
  | MallTask
  | ReclamationAlgorithmTask
  | RecruitTask
  | RoguelikeTask
  | StartUpTask

export type { Task, FightTask, StartUpTask }
