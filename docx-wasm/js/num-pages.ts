import * as wasm from "./pkg/docx_wasm";

export class NumPages {
  build() {
    return wasm.createNumPages();
  }
}
