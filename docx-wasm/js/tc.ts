export class Tc {
  _text: string;
  _level?: number | undefined;
  _omitPageNumber: boolean;
  _identifier?: string | undefined;

  constructor(t: string) {
    this._text = t;
  }

  level(l: number) {
    this._level = l;
    return this;
  }

  omitPageNumber() {
    this._omitPageNumber = true;
    return this;
  }

  identifier(id: string) {
    this._identifier = id;
    return this;
  }
}
