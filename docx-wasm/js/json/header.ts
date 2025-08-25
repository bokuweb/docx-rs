import { ParagraphJSON } from "./paragraph";
import { StructuredTagJSON } from "./structured-data-tag";
import { TableJSON } from "./table";

export type HeaderJSON = [
  string,
  { children: (ParagraphJSON | TableJSON | StructuredTagJSON)[] }
];

export type HeaderReferenceJSON = {
  headerType: string;
  id: string;
};
