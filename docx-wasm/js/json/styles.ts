import { RunPropertyJSON } from "./run";
import { ParagraphPropertyJSON } from "./paragraph";

export type StyleJSON = {
  styleId: string;
  name: string;
  styleType: string;
  runProperty: RunPropertyJSON;
  paragraphProperty: ParagraphPropertyJSON;
};

export type Styles = {
  docDefaults: {
    runPropertyDefault: RunPropertyJSON;
  };
  styles: StyleJSON[];
};
