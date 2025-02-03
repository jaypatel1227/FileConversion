import React, { useState } from "react";
import * as Utils from "./Utils";

export interface ISelectorModeParams {
  services: Utils.IServices,
}

export const SelectorMode: React.FC<ISelectorModeParams> = (props) => {
  let [from, setFrom] = useState<string>("");
  let [to, setTo] = useState<string>("");

  return (
    <div className="_selectorMode">
      <div> From: </div>
      <input className="_selectionBox" type="text" placeholder="🔍" value={from} onChange={(e) => setFrom(e.target.value)} />
      <div> To: </div>
      <input className="_selectionBox" type="text" placeholder="🔍" value={to} onChange={(e) => setTo(e.target.value)} />
    </div>
  );
}

export default SelectorMode;
