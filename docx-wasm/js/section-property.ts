import { DocGridType } from ".";

export type DocGrid = {
  gridType: DocGridType;
  linePitch?: number;
  charSpace?: number;
};

export class SectionProperty {
  _pageSize: PageSize = {
    w: 11906,
    h: 16838,
  };
  _pageMargin: PageMargin | null = null;
  _docGrid: DocGrid = {
    gridType: "lines",
    linePitch: 360,
  };

  pageSize(w: number, h: number) {
    this._pageSize.w = w;
    this._pageSize.h = h;
    return this;
  }

  pageMargin(margin: Partial<PageMargin>) {
    this._pageMargin = { ...defaultPageMargin(), ...margin };
    return this;
  }

  pageOrientation(orient: PageOrientationType) {
    this._pageSize.orient = orient;
    return this;
  }

  docGrid(gridType: DocGridType, linePitch?: number, charSpace?: number) {
    this._docGrid = { gridType, linePitch, charSpace };
    return this;
  }
}

export type PageOrientationType = "landscape" | "portrait";

export type PageSize = { w: number; h: number; orient?: PageOrientationType };

export type PageMargin = {
  top: number;
  right: number;
  bottom: number;
  left: number;
  header: number;
  footer: number;
  gutter: number;
};

export const defaultPageMargin = () => {
  return {
    top: 1985,
    left: 1701,
    bottom: 1701,
    right: 1701,
    header: 851,
    footer: 992,
    gutter: 0,
  };
};
