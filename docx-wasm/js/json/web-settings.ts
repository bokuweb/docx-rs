export type WebSettingsJSON = {
  divs: DivJSON[];
};

export type DivJSON = {
  id: string;
  marginLeft: number;
  marginRight: number;
  marginTop: number;
  marginBottom: number;
  divsChild: DivJSON | null;
};
