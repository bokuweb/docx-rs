export type BreakType = "page" | "column" | "textWrapping";

export class Break {
  type: BreakType;
  constructor(type: BreakType) {
    this.type = type;
  }
}
