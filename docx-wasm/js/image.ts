export class Image {
  data: Uint8Array;
  w: number | null = null;
  h: number | null = null;
  _floating: boolean = false;
  _offsetX = 0;
  _offsetY = 0;

  constructor(data: Uint8Array) {
    this.data = data;
  }

  size = (w: number, h: number) => {
    this.w = w;
    this.h = h;
  };

  floating = () => {
    this._floating = true;
  };

  offsetX = (x: number) => {
    this._offsetX = x;
  };

  offsetY = (y: number) => {
    this._offsetY = y;
  };
}
