import { FrameProperty } from "./page-num";
import {
  AlignmentType,
  ParagraphProperty,
  createDefaultParagraphProperty,
  createParagraphAlignment,
} from "./paragraph-property";
import * as wasm from "./pkg/docx_wasm";

export class NumPages {
  frameProperty: FrameProperty | null = null;
  paragraphProperty: ParagraphProperty | null = null;

  height(h: number) {
    this.frameProperty = { ...this.frameProperty };
    this.frameProperty.h = h;
    return this;
  }
  hRule(r: string) {
    this.frameProperty = { ...this.frameProperty };
    this.frameProperty.hRule = r;
    return this;
  }

  hAnchor(a: string) {
    this.frameProperty = { ...this.frameProperty };
    this.frameProperty.hAnchor = a;
    return this;
  }

  hSpace(s: number) {
    this.frameProperty = { ...this.frameProperty };
    this.frameProperty.hSpace = s;
    return this;
  }

  vAnchor(a: string) {
    this.frameProperty = { ...this.frameProperty };
    this.frameProperty.vAnchor = a;
    return this;
  }

  vSpace(s: number) {
    this.frameProperty = { ...this.frameProperty };
    this.frameProperty.vSpace = s;
    return this;
  }

  width(w: number) {
    this.frameProperty = { ...this.frameProperty };
    this.frameProperty.w = w;
    return this;
  }

  wrap(w: string) {
    this.frameProperty = { ...this.frameProperty };
    this.frameProperty.wrap = w;
    return this;
  }

  x(x: number) {
    this.frameProperty = { ...this.frameProperty };
    this.frameProperty.x = x;
    return this;
  }

  xAlign(a: string) {
    this.frameProperty = { ...this.frameProperty };
    this.frameProperty.xAlign = a;
    return this;
  }

  y(y: number) {
    this.frameProperty = { ...this.frameProperty };
    this.frameProperty.y = y;
    return this;
  }

  yAlign(y: string) {
    this.frameProperty = { ...this.frameProperty };
    this.frameProperty.yAlign = y;
    return this;
  }

  align(align: AlignmentType) {
    this.paragraphProperty = {
      ...createDefaultParagraphProperty(),
      align,
    };
    return this;
  }

  build() {
    let numPages = wasm.createNumPages();
    if (this.frameProperty?.h != null) {
      numPages = numPages.height(this.frameProperty.h);
    }
    if (this.frameProperty?.hRule != null) {
      numPages = numPages.h_rule(this.frameProperty.hRule);
    }
    if (this.frameProperty?.hAnchor != null) {
      numPages = numPages.h_anchor(this.frameProperty.hAnchor);
    }
    if (this.frameProperty?.hSpace != null) {
      numPages = numPages.h_space(this.frameProperty.hSpace);
    }
    if (this.frameProperty?.vAnchor != null) {
      numPages = numPages.v_anchor(this.frameProperty.vAnchor);
    }
    if (this.frameProperty?.vSpace != null) {
      numPages = numPages.v_space(this.frameProperty.vSpace);
    }
    if (this.frameProperty?.w != null) {
      numPages = numPages.width(this.frameProperty.w);
    }
    if (this.frameProperty?.wrap != null) {
      numPages = numPages.wrap(this.frameProperty.wrap);
    }
    if (this.frameProperty?.x != null) {
      numPages = numPages.x(this.frameProperty.x);
    }
    if (this.frameProperty?.xAlign != null) {
      numPages = numPages.x_align(this.frameProperty.xAlign);
    }
    if (this.frameProperty?.y != null) {
      numPages = numPages.y(this.frameProperty.y);
    }
    if (this.frameProperty?.yAlign != null) {
      numPages = numPages.y_align(this.frameProperty.yAlign);
    }
    if (this.paragraphProperty?.align != null) {
      const align = createParagraphAlignment(this.paragraphProperty.align);
      if (align) {
        numPages = numPages.align(align);
      }
    }
    return numPages;
  }
}
