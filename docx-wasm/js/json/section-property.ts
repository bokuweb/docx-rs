import { FooterReference } from "./bindings/FooterReference";
import { HeaderReference } from "./bindings/HeaderReference";

export type DocGridType = "default" | "lines" | "linesAndChars" | "snapToChars";

export type SectionType =
  | "nextPage"
  | "nextColumn"
  | "continuous"
  | "evenPage"
  | "oddPage";

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
  sectionType?: SectionType;
  headerReference?: HeaderReference;
  footerReference?: FooterReference;
};
