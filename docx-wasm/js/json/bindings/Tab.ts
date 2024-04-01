import type { TabLeaderType } from "./TabLeaderType";
import type { TabValueType } from "./TabValueType";

export interface Tab { val: TabValueType | null, leader: TabLeaderType | null, pos: number | null, }