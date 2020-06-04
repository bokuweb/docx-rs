import { LevelOverride } from "./level";

export class Numbering {
  id: number;
  abstractNumId: number;
  overrides: LevelOverride[] = [];

  constructor(id: number, abstractNumId: number) {
    this.id = id;
    this.abstractNumId = abstractNumId;
  }

  addOverride(o: LevelOverride) {
    this.overrides.push(o);
  }
}
