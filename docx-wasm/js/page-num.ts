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

  build() {
    let pageNum = wasm.createPageNum();
    pageNum = etTableProperty(pageNum, this.property);
    return pageNum;
  }
}
