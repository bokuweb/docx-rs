export class Shading {
  _type: string = "clear";
  _color: string = "auto";
  _fill: string = "FFFFFF";

  color(c: string) {
    this._color = c;
  }

  fill(c: string) {
    this._fill = c;
  }
}
