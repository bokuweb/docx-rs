export class Image {
  data: Uint8Array;
  w: number | null = null;
  h: number | null = null;
  _floating: boolean = false;
  _offsetX = 0;
  _offsetY = 0;
  rot = 0;

  constructor(data: Uint8Array) {
    this.data = data;
  }

  size = (w: number, h: number) => {
    this.w = w;
    this.h = h;
    return this;
  };

  rotate = (deg: number) => {
    this.rot = deg;
    return this;
  };

  floating = () => {
    this._floating = true;
    return this;
  };

  offsetX = (x: number) => {
    this._offsetX = x;
    return this;
  };

  offsetY = (y: number) => {
    this._offsetY = y;
    return this;
  };
}
