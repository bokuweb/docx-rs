import { RunProperty, RunFonts } from "./run";

export class DocDefaults {
  runProperty: RunProperty;

  size(size: number) {
    this.runProperty = { ...this.runProperty, size };
    return this;
  }

  fonts(fonts: RunFonts) {
    this.runProperty = { ...this.runProperty, fonts };
    return this;
  }

  characterSpacing(characterSpacing: number) {
    this.runProperty = { ...this.runProperty, characterSpacing };
    return this;
  }
}
