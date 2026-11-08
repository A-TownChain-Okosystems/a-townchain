// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
const fs = require('fs');

const file = 'src/App.tsx';
let content = fs.readFileSync(file, 'utf8');

const replacements = [
  // Backgrounds
  [/bg-slate-950/g, 'bg-slate-50'],
  [/bg-slate-900/g, 'bg-white'],
  [/bg-slate-800/g, 'bg-slate-100'],
  
  // Texts
  [/text-white/g, 'text-slate-900'],
  [/text-slate-200/g, 'text-slate-800'],
  [/text-slate-300/g, 'text-slate-600'],
  [/text-slate-400/g, 'text-slate-500'],
  [/text-slate-500/g, 'text-slate-400'],
  [/text-slate-600/g, 'text-slate-300'],
  
  // Borders
  [/border-slate-800/g, 'border-slate-200'],
  [/border-slate-700/g, 'border-slate-300'],
  [/border-white\/5/g, 'border-slate-200'],
  [/border-white\/10/g, 'border-slate-300'],
  [/bg-white\/\[0\.02\]/g, 'bg-white'],
  [/bg-white\/\[0\.04\]/g, 'bg-slate-50'],
  [/bg-white\/5/g, 'bg-slate-50'],
  
  // Colors (Cyan -> Blue)
  [/cyan-400/g, 'blue-600'],
  [/cyan-500/g, 'blue-500'],
  [/cyan-900/g, 'blue-100'],
  [/cyan-950/g, 'blue-50'],
  
  // Colors (Emerald -> Green)
  [/emerald-400/g, 'green-600'],
  [/emerald-500/g, 'green-500'],
  [/emerald-900/g, 'green-100'],
  [/emerald-950/g, 'green-50'],
  
  // Colors (Indigo -> Indigo)
  [/indigo-400/g, 'indigo-600'],
  [/indigo-500/g, 'indigo-500'],
  [/indigo-900/g, 'indigo-100'],
  [/indigo-950/g, 'indigo-50'],
  
  // Colors (Purple -> Purple)
  [/purple-400/g, 'purple-600'],
  [/purple-500/g, 'purple-500'],
  [/purple-900/g, 'purple-100'],
  [/purple-950/g, 'purple-50'],
  
  // Colors (Amber -> Orange)
  [/amber-400/g, 'orange-600'],
  [/amber-500/g, 'orange-500'],
  [/amber-900/g, 'orange-100'],
  [/amber-950/g, 'orange-50'],

  // Colors (Fuchsia -> Pink)
  [/fuchsia-400/g, 'pink-600'],
  [/fuchsia-500/g, 'pink-500'],
  [/fuchsia-900/g, 'pink-100'],
  [/fuchsia-950/g, 'pink-50'],

  // Opacities inside bg-color/[opacity] are mostly fine, but we need to ensure text contrasts
  [/text-slate-900 hover:text-slate-900/g, 'text-slate-600 hover:text-slate-900'],
  
  // Titles / Brand
  [/Master Architecture v2/g, 'Enterprise Knowledge Base'],
  
  // Specific shadow for dots
  [/shadow-\[0_0_8px_rgba\(245,158,11,0\.6\)\]/g, 'shadow-[0_0_8px_rgba(245,158,11,0.2)]'],
  [/shadow-\[0_0_8px_rgba\(6,182,212,0\.6\)\]/g, 'shadow-[0_0_8px_rgba(37,99,235,0.2)]'],
  [/shadow-\[0_0_8px_rgba\(16,185,129,0\.6\)\]/g, 'shadow-[0_0_8px_rgba(34,197,94,0.2)]'],
  [/shadow-\[0_0_20px_rgba\(6,182,212,0\.1\)\]/g, 'shadow-[0_0_20px_rgba(37,99,235,0.1)]'],
  
  // Clean background mix
  [/bg-gradient-to-br from-indigo-50\/40 to-blue-50\/20/g, 'bg-gradient-to-br from-slate-50 to-white'],
  [/bg-gradient-to-br from-white to-black/g, 'bg-gradient-to-br from-slate-50 to-white'],
  [/bg-gradient-to-br from-green-50\/20 to-black/g, 'bg-gradient-to-br from-green-50 to-white'],
  [/bg-gradient-to-r from-pink-50\/20 to-transparent/g, 'bg-gradient-to-r from-pink-50 to-transparent'],
  [/bg-gradient-to-r from-indigo-50\/20 to-transparent/g, 'bg-gradient-to-r from-indigo-50 to-transparent'],
  [/bg-gradient-to-br from-slate-50 to-black/g, 'bg-slate-50'],
  
  // Pre blocks background
  [/bg-\[\#0a0a0f\]/g, 'bg-slate-800'],
  
  // Update header text from "System Network // System OS" to "Enterprise Wiki"
  [/System Network \/\/ System OS/g, "Enterprise Wiki"],
  
  // Change background black
  [/to-black/g, 'to-slate-100']
];

let newContent = content;
replacements.forEach(([regex, replacement]) => {
  newContent = newContent.replace(regex, replacement);
});

fs.writeFileSync(file, newContent, 'utf8');
console.log('Replacements applied to App.tsx');
