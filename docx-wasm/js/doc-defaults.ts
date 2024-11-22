import { LineSpacing, ParagraphProperty } from "./paragraph-property";
import {
  RunProperty,
  RunFonts,
  createDefaultRunProperty,
} from "./run-property";

export class DocDefaults {
  runProperty: RunProperty;
  paragraphProperty: ParagraphProperty = new ParagraphProperty();

  size(size: number) {
    this.runProperty ??= createDefaultRunProperty();
    this.runProperty.size(size);
    return this;
  }

  fonts(fonts: RunFonts) {
    this.runProperty ??= createDefaultRunProperty();
    this.runProperty.fonts(fonts);
    return this;
  }

  characterSpacing(characterSpacing: number) {
    this.runProperty ??= createDefaultRunProperty();
    this.runProperty.spacing(characterSpacing);
    return this;
  }

  lineSpacing(lineSpacing: LineSpacing) {
    this.paragraphProperty.lineSpacing = lineSpacing;
    return this;
  }
}
