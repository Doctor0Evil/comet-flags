import React from "react";
import { FlagModel } from "./FlagSection";

interface Props {
  flag: FlagModel;
}

const FlagEntry: React.FC<Props> = ({ flag }) => (
  <div className="flag-entry">
    <p className="flag-title">{flag.title}</p>
    <p className="flag-description">{flag.description}</p>
    <span className="flag-id">{flag.id}</span>
  </div>
);

export default FlagEntry;
