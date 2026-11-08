import React, { useState } from 'react';
import { HelpCircle } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

export function TooltipIcon({ content }: { content: string }) {
  const [isHovered, setIsHovered] = useState(false);

  return (
    <div 
      className="relative inline-flex items-center justify-center ml-2"
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      <HelpCircle className="w-4 h-4 text-slate-500 hover:text-atc-cyan transition-colors cursor-help" />
      <AnimatePresence>
        {isHovered && (
          <motion.div
            initial={{ opacity: 0, y: 5 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: 5 }}
            className="absolute bottom-full mb-2 left-1/2 -translate-x-1/2 w-48 p-2 bg-slate-800 text-slate-200 text-xs rounded-lg shadow-xl border border-slate-700 z-50 text-center font-sans tracking-normal"
          >
            {content}
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
