import { Style } from "./style";
import { DocDefaults } from "./doc-defaults";
import { RunFonts } from "./run";

export class Styles {
  styles: Style[] = [];
  docDefaults = new DocDefaults();

  defaultSize(size: number) {
    this.docDefaults.size(size);
    return this;
  }

  defaultFonts(fonts: RunFonts) {
    this.docDefaults.fonts(fonts);
    return this;
  }

  defaultCharacterSpacing(spacing: number) {
    this.docDefaults.characterSpacing(spacing);
    return this;
  }
}
