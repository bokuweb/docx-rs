import { PositionalTabAlignmentType } from "./json/bindings/PositionalTabAlignmentType";
import { PositionalTabRelativeTo } from "./json/bindings/PositionalTabRelativeTo";
import { TabLeaderType } from "./json/bindings/TabLeaderType";
import {
  createPositionalTab,
  PositionalTabAlignmentType as _PositionalTabAlignmentType,
  PositionalTabRelativeTo as _PositionalTabRelativeTo,
  TabLeaderType as _TabLeaderType,
} from "./pkg/docx_wasm";
import { convertTabLeader } from "./tab-leader";

export class PositionalTab {
  _alignment: PositionalTabAlignmentType = "left";
  _relativeTo: PositionalTabRelativeTo = "margin";
  _leader: TabLeaderType = "none";

  alignment(t: PositionalTabAlignmentType) {
    this._alignment = t;
    return this;
  }

  relativeTo(t: PositionalTabRelativeTo) {
    this._relativeTo = t;
    return this;
  }

  leader(l: TabLeaderType) {
    this._leader = l;
    return this;
  }

  buildWasmObject() {
    const alignment = (() => {
      if (this._alignment === "left") return _PositionalTabAlignmentType.Left;
      if (this._alignment === "center")
        return _PositionalTabAlignmentType.Center;
      if (this._alignment === "right") return _PositionalTabAlignmentType.Right;
      return _PositionalTabAlignmentType.Left;
    })();

    const relativeTo = (() => {
      if (this._relativeTo === "indent") return _PositionalTabRelativeTo.Indent;
      return _PositionalTabRelativeTo.Margin;
    })();

    const leader = convertTabLeader(this._leader);

    return createPositionalTab(alignment, relativeTo, leader);
  }
}
