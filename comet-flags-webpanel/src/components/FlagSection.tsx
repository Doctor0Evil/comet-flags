import React from "react";
import FlagEntry from "./FlagEntry";

export interface FlagModel {
  id: string;
  title: string;
  description: string;
}

interface Props {
  title: string;
  flags: FlagModel[];
}

const FlagSection: React.FC<Props> = ({ title, flags }) => (
  <div className="flag-section">
    <h2>{title}</h2>
    {flags.map((flag) => (
      <FlagEntry key={flag.id} flag={flag} />
    ))}
  </div>
);

export default FlagSection;
