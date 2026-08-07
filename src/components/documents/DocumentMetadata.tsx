import "./DocumentMetadata.css";

export interface MetadataEntry {
  label: string;
  value: string;
  mono?: boolean;
}

interface DocumentMetadataProps {
  entries: MetadataEntry[];
}

export default function DocumentMetadata({ entries }: DocumentMetadataProps) {
  return (
    <section
      className="pqm-document-metadata"
      aria-label="Dokumentmetadaten"
    >
      <h3 className="pqm-document-metadata__heading">Metadaten</h3>
      <dl className="pqm-document-metadata__grid">
        {entries.map((entry) => (
          <div key={entry.label} className="pqm-document-metadata__row">
            <dt className="pqm-document-metadata__label">{entry.label}</dt>
            <dd
              className={
                "pqm-document-metadata__value" +
                (entry.mono ? " pqm-document-metadata__value--mono" : "")
              }
            >
              {entry.value}
            </dd>
          </div>
        ))}
      </dl>
    </section>
  );
}
