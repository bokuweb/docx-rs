import { ParagraphJSON } from "./paragraph";
import { TableJSON } from "./table";

export type FooterJSON = {
  children: (ParagraphJSON | TableJSON)[];
};

export type FooterReferenceJSON = {
  footerType: string;
  id: string;
};
