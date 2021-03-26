import { RunPropertyJSON } from "./run";
import { ParagraphPropertyJSON } from "./paragraph";
import { TablePropertyJSON } from "./table";
import { TableCellPropertyJSON } from "..";

export type StyleJSON = {
  styleId: string;
  name: string;
  styleType: string;
  runProperty: RunPropertyJSON;
  paragraphProperty: ParagraphPropertyJSON;
  tableProperty: TablePropertyJSON;
  tableCellProperty: TableCellPropertyJSON;
  basedOn: string | null;
};

export type StylesJSON = {
  docDefaults: {
    runPropertyDefault: {
      runProperty: RunPropertyJSON;
    };
  };
  styles: StyleJSON[];
};
