import React from 'react';

// Search Bar — Full-text search across whitepaper chapters
export default function SearchBar({ onSearch }: { onSearch: (query: string) => void }) {
  return (
    <div className="search-bar">
      <input type="text" placeholder="Search whitepaper..." onChange={(e) => onSearch(e.target.value)} />
    </div>
  );
}
