export type DocGridType = "default" | "lines" | "linesAndChars" | "snapToChars";

export type DocGridJSON = {
  gridType: DocGridType;
  linePitch: number | null;
  charSpace: number | null;
};

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
  docGrid?: DocGridJSON;
};
