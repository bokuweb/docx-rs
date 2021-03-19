export class Shading {
  _type: string = "clear";
  _color: string = "auto";
  _fill: string = "FFFFFF";

  type(t: string) {
    this._type = t;
  }

  color(c: string) {
    this._color = c;
  }

  fill(c: string) {
    this._fill = c;
  }
}
