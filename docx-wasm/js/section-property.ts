export class SectionProperty {
  _pageSize: PageSize | null = null;
  _pageMargin: PageMargin | null = null;

  pageSize(w: number, h: number) {
    this._pageSize = { w, h };
    return this;
  }

  pageMargin(margin: Partial<PageMargin>) {
    this._pageMargin = { ...defaultPageMargin(), ...margin };
    return this;
  }
}

export type PageSize = { w: number; h: number };

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
