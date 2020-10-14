export type SectionPropertyJSON = {
  pageSize: {
    w: number;
    h: number;
  };
  pageMargin: {
    top: number;
    left: number;
    bottom: number;
    right: number;
    header: number;
    footer: number;
    gutter: number;
  };
  columns: number;
  documentGrid: number;
};
