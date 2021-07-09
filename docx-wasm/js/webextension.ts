export class WebExtension {
  _id: string;
  _referenceId: string;
  _version: string;
  _store: string;
  _storeType: string;
  properties: {
    [k: string]: string;
  } = {};

  constructor(
    id: string,
    referenceId: string,
    version: string,
    store: string,
    storeType: string
  ) {
    this._id = id;
    this._referenceId = referenceId;
    this._version = version;
    this._store = store;
    this._storeType = storeType;
  }

  property(k: string, v: string) {
    this.properties[k] = v;
    return this;
  }
}
