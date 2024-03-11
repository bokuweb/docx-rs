import * as wasm from "./pkg/docx_wasm";

export type FrameProperty = {
  h?: number;
  hRule?: string;
  hAnchor?: string;
  hSpace?: number;
  vAnchor?: string;
  vSpace?: number;
  w?: number;
  wrap?: string;
  x?: number;
  xAlign?: string;
  y?: number;
  yAlign?: string;
};

export class PageNum {
  frameProperty: FrameProperty = {};

  height(h: number) {
    this.frameProperty.h = h;
    return this;
  }
  hRule(r: string) {
    this.frameProperty.hRule = r;
    return this;
  }

  hAnchor(a: string) {
    this.frameProperty.hAnchor = a;
    return this;
  }

  hSpace(s: number) {
    this.frameProperty.hSpace = s;
    return this;
  }

  vAnchor(a: string) {
    this.frameProperty.vAnchor = a;
    return this;
  }

  vSpace(s: number) {
    this.frameProperty.vSpace = s;
    return this;
  }

  width(w: number) {
    this.frameProperty.w = w;
    return this;
  }

  wrap(w: string) {
    this.frameProperty.wrap = w;
    return this;
  }

  x(x: number) {
    this.frameProperty.x = x;
    return this;
  }

  xAlign(a: string) {
    this.frameProperty.xAlign = a;
    return this;
  }

  y(y: number) {
    this.frameProperty.y = y;
    return this;
  }

  yAlign(y: string) {
    this.frameProperty.yAlign = y;
    return this;
  }

  build() {
    let pageNum = wasm.createPageNum();
    if (this.frameProperty.h != null) {
      pageNum = pageNum.height(this.frameProperty.h);
    }
    if (this.frameProperty.hRule != null) {
      pageNum = pageNum.h_rule(this.frameProperty.hRule);
    }
    if (this.frameProperty.hAnchor != null) {
      pageNum = pageNum.h_anchor(this.frameProperty.hAnchor);
    }
    if (this.frameProperty.hSpace != null) {
      pageNum = pageNum.h_space(this.frameProperty.hSpace);
    }
    if (this.frameProperty.vAnchor != null) {
      pageNum = pageNum.v_anchor(this.frameProperty.vAnchor);
    }
    if (this.frameProperty.vSpace != null) {
      pageNum = pageNum.v_space(this.frameProperty.vSpace);
    }
    if (this.frameProperty.w != null) {
      pageNum = pageNum.width(this.frameProperty.w);
    }
    if (this.frameProperty.wrap != null) {
      pageNum = pageNum.wrap(this.frameProperty.wrap);
    }
    if (this.frameProperty.x != null) {
      pageNum = pageNum.x(this.frameProperty.x);
    }
    if (this.frameProperty.xAlign != null) {
      pageNum = pageNum.x_align(this.frameProperty.xAlign);
    }
    if (this.frameProperty.y != null) {
      pageNum = pageNum.y(this.frameProperty.y);
    }
    if (this.frameProperty.yAlign != null) {
      pageNum = pageNum.y_align(this.frameProperty.yAlign);
    }
    return pageNum;
  }
}
