import { Level } from "./level";

export class Numbering {
  id: number;
  levels: Level[];
  constructor(id: number) {
    this.id = id;
  }
  addLevel(level: Level) {
    this.levels.push(level);
    return this;
  }
}
