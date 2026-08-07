import { Tag } from "lucide-react";
import "./TagList.css";

interface TagListProps {
  tags: string[];
}

export default function TagList({ tags }: TagListProps) {
  return (
    <section className="pqm-tag-list" aria-label="Schlagwörter">
      <h3 className="pqm-tag-list__heading">
        <Tag size={16} aria-hidden="true" />
        Schlagwörter
      </h3>
      <div className="pqm-tag-list__tags">
        {tags.map((tag) => (
          <span key={tag} className="pqm-tag-list__tag">
            {tag}
          </span>
        ))}
      </div>
    </section>
  );
}
