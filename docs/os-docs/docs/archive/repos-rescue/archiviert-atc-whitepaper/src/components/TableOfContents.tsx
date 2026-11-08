import React from 'react';

// Table of Contents — Hierarchical TOC with section links
export default function TableOfContents({ sections }: { sections: { id: string; title: string; level: number }[] }) {
  return (
    <div className="toc">
      <h3>Table of Contents</h3>
      <ul>
        {sections.map((s) => (
          <li key={s.id} style={{ marginLeft: (s.level - 1) * 16 }}>
            <a href={`#${s.id}`}>{s.title}</a>
          </li>
        ))}
      </ul>
    </div>
  );
}
