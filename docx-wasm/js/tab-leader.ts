import * as wasm from "./pkg";
import { TabLeaderType } from "./json/bindings/TabLeaderType";

export const convertTabLeader = (leader: TabLeaderType) => {
  switch (leader) {
    case "dot":
      return wasm.TabLeaderType.Dot;
      break;
    case "heavy":
      return wasm.TabLeaderType.Heavy;
    case "hyphen":
      return wasm.TabLeaderType.Hyphen;
    case "middleDot":
      return wasm.TabLeaderType.MiddleDot;
    case "none":
      return wasm.TabLeaderType.None;
    case "underscore":
      return wasm.TabLeaderType.Underscore;
    default:
      return wasm.TabLeaderType.None;
  }
};
