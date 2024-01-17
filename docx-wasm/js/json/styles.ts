import { RunPropertyJSON } from "./run";
import { ParagraphPropertyJSON } from "./paragraph";
import { TablePropertyJSON } from "./table";
import { TableCellPropertyJSON } from "..";
import { StyleType } from "../style";

export type StyleJSON = {
  styleId: string;
  name: string;
  styleType: StyleType;
  runProperty: RunPropertyJSON;
  paragraphProperty: ParagraphPropertyJSON;
  tableProperty: TablePropertyJSON;
  tableCellProperty: TableCellPropertyJSON;
  basedOn: string | null;
  link?: string | null | undefined;
  next?: string | null;
};

export type StylesJSON = {
  docDefaults: {
    runPropertyDefault: {
      runProperty: RunPropertyJSON;
    };
    paragraphPropertyDefault: {
      paragraphProperty: ParagraphPropertyJSON;
    };
  };
  styles: StyleJSON[];
};
