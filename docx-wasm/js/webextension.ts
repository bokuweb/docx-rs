export class WebExtension {
  _id: string;
  _version: string;
  _store: string;
  _storeType: string;
  properties: {
    [k: string]: string;
  } = {};

  constructor(id: string, version: string, store: string, storeType: string) {
    this._id = id;
    this._version = version;
    this._store = store;
    this._storeType = storeType;
  }

  property(k: string, v: string) {
    this.properties[k] = v;
    return this;
  }
}
