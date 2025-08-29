import { ParagraphJSON } from "./paragraph";
import { StructuredTagJSON } from "./structured-data-tag";
import { TableJSON } from "./table";

export type FooterJSON = [
  string,
  {
    children: (ParagraphJSON | TableJSON | StructuredTagJSON)[];
  }
];

export type FooterReferenceJSON = {
  footerType: string;
  id: string;
};
