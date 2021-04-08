import { StylesJSON } from "./styles";
import { DocumentJSON } from "./document";
import { NumberingsJSON } from "./numbering";
import { CommentJSON } from "./comment";
import { WebSettingsJSON } from "./web-settings";

export type DocxJSON = {
  contentType: {
    types: {
      [k: string]: string;
    };
  };
  rels: [string, string, string][];
  documentRels: {
    hasComments: boolean;
    hasNumberings: boolean;
  };
  docProps: {
    app: {};
    core: {
      config: {
        creator: string | null;
        description: string | null;
        language: string | null;
        lastModifiedBy: string | null;
        modified: string | null;
        revision: string | null;
        subject: string | null;
        title: string | null;
      };
    };
  };
  styles: StylesJSON;
  document: DocumentJSON;
  comments: {
    comments: CommentJSON[];
  };
  numberings: NumberingsJSON;
  settings: SettingsJSON;
  webSettings: WebSettingsJSON;
  fontTable: {};
};

export type SettingsJSON = {
  // w15:docId
  docId: string | null;
  defaultTabStop: number;
  zoom: number;
  docVars: { name: string; val: string }[];
};

export * from "./styles";
export * from "./border";
export * from "./document";
export * from "./paragraph";
export * from "./run";
export * from "./table";
export * from "./numbering";
export * from "./drawing";
export * from "./shading";
export * from "./web-settings";
export * from "./comment";
export * from "./textbox-content";
export * from "./section-property";
