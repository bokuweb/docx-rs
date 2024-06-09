import * as wasm from "./pkg/docx_wasm";

export class PageNum {
  build() {
    const pageNum = wasm.createPageNum();
    return pageNum;
  }
}
