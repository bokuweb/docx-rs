import { LineSpacing, ParagraphProperty } from "./paragraph-property";
import { RunProperty, RunFonts } from "./run";

export class DocDefaults {
  runProperty: RunProperty;
  paragraphProperty: ParagraphProperty;

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

  lineSpacing(lineSpacing: LineSpacing) {
    this.paragraphProperty = { ...this.paragraphProperty, lineSpacing };
    return this;
  }
}
