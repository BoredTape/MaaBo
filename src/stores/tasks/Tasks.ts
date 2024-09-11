import { type AwardTask } from './Award'
import { type FightTask } from './Fight'
import { type InfrastTask } from './Infrast'
import { type MallTask } from './Mall'
import { type ReclamationTask } from './Reclamation'
import { type RecruitTask } from './Recruit'
import { type RoguelikeTask } from './Roguelike'
import { type StartUpTask } from './StartUp'

type Task =
  | AwardTask
  | FightTask
  | InfrastTask
  | MallTask
  | ReclamationTask
  | RecruitTask
  | RoguelikeTask
  | StartUpTask

export type { Task, FightTask, StartUpTask }
