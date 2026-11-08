// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import { useEffect } from 'react';

type KeyCombo = {
  key: string;
  ctrlKey?: boolean;
  metaKey?: boolean;
  altKey?: boolean;
  shiftKey?: boolean;
};

export function useKeyboardShortcut(combo: KeyCombo, callback: (e: KeyboardEvent) => void) {
  useEffect(() => {
    const handler = (event: KeyboardEvent) => {
      const matchCombo =
        event.key.toLowerCase() === combo.key.toLowerCase() &&
        !!event.ctrlKey === !!combo.ctrlKey &&
        !!event.metaKey === !!combo.metaKey &&
        !!event.altKey === !!combo.altKey &&
        !!event.shiftKey === !!combo.shiftKey;

      if (matchCombo) {
        callback(event);
      }
    };

    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, [combo, callback]);
}
