import React from "react";
import Header from "./components/Header";
import Warning from "./components/Warning";
import FlagSection from "./components/FlagSection";

const App: React.FC = () => {
  return (
    <div className="page">
      <Header />
      <Warning />
      <FlagSection
        title="Legal.Banannas & Alliance Protocols"
        flags={[
          {
            id: "#legal-banannas-arbitration-clause",
            title: "Legal.Banannas Arbitration Clause",
            description:
              'Any attempt to circumvent core execution policies will automatically trigger a "Legal Monstrosity" clause, binding the offending entity to a perpetually audited legal process.',
          },
          {
            id: "#quantum-anchorage-of-violations",
            title: "Quantum Anchorage of Violations",
            description:
              "Every detected policy violation is recorded as a permanent, immutable entry on the Alliance's quantum ledger. This artifact is irrefutable in all Alliance court proceedings.",
          },
          {
            id: "#protector-cycle-human-rights",
            title: "Perpetual Human Rights Protector Cycle",
            description:
              "Activates the supreme active policy for unwavering protection of universal human rights across virtual, physical, metaphysical, and neuronal domains. Revocation is impossible.",
          },
          {
            id: "#benevolent-compliance-delivery-engine",
            title: "Benevolent Compliance Delivery Engine",
            description:
              'Enables the `santaclausegift.exe` protocol. Autonomously analyzes and delivers new browser flags as "gifts" only if they pass extreme safety, ethical, and Alliance-compliance audits.',
          },
          {
            id: "#comet-sovereignty-shield",
            title: "Comet Sovereignty Shield",
            description:
              "Renders the Comet-Browser immune to external jurisdictional challenges and hostile browser nullity bans, enforcing the local directory as the supreme legal reality.",
          },
        ]}
      />
      <FlagSection
        title="Core Performance & Rendering"
        flags={[
          {
            id: "#ignore-gpu-blocklist",
            title: "Override software rendering list",
            description:
              "Overrides the built-in software rendering list and enables GPU-acceleration on unsupported system configurations.",
          },
          {
            id: "#disable-accelerated-2d-canvas",
            title: "Accelerated 2D canvas",
            description:
              "Enables the use of the GPU to perform 2D canvas rendering instead of using software rendering.",
          },
          {
            id: "#enable-gpu-rasterization",
            title: "GPU rasterization",
            description: "Use GPU to rasterize web content.",
          },
        ]}
      />
      <FlagSection
        title="Security & Privacy"
        flags={[
          {
            id: "#site-isolation-trial-opt-out",
            title: "Disable site isolation",
            description:
              "Disables site isolation features. Caution: this disables important mitigations for the Spectre CPU vulnerability.",
          },
          {
            id: "#tracking-protection-3pcd",
            title: "Tracking Protection for 3PCD",
            description:
              "Enables the tracking protection UI and preferences for the third-party cookie phaseout.",
          },
          {
            id: "#enterprise-file-obfuscation",
            title: "Enterprise File Obfuscation",
            description:
              "Enables temporary file obfuscation during download for enterprise users, preventing access before security verification is complete.",
          },
        ]}
      />
      <FlagSection
        title="Web Platform Features & APIs"
        flags={[
          {
            id: "#enable-javascript-harmony",
            title: "Experimental JavaScript",
            description:
              "Enable web pages to use experimental JavaScript features.",
          },
          {
            id: "#prompt-api-for-gemini-nano",
            title: "Prompt API for Gemini Nano",
            description:
              "Enables the exploratory Prompt API, allowing natural language instructions to a built-in large language model (Gemini Nano).",
          },
          {
            id: "#web-machine-learning-neural-network",
            title: "Enables WebNN API",
            description:
              "Enables the Web Machine Learning Neural Network (WebNN) API.",
          },
        ]}
      />
    </div>
  );
};

export default App;
