import React from 'react';

// Chapter Navigation — Sidebar with chapter list and progress
export default function ChapterNavigation({ chapters, current, onSelect }: {
  chapters: { id: string; title: string }[];
  current: string;
  onSelect: (id: string) => void;
}) {
  return (
    <nav className="chapter-nav">
      <h3>Chapters</h3>
      <ul>
        {chapters.map((ch, i) => (
          <li key={ch.id} className={ch.id === current ? 'active' : ''} onClick={() => onSelect(ch.id)}>
            {i + 1}. {ch.title}
          </li>
        ))}
      </ul>
    </nav>
  );
}
