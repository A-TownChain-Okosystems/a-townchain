export const FOLD_MARKER = (id: number) => `/* ⬇ FOLDED_BLOCK_${id} ⬇ */`;

export interface FoldRegion {
    start: number;
    end: number;
}

export const getFoldRegions = (code: string): Map<number, FoldRegion> => {
    const lines = code.split('\n');
    const regions = new Map<number, FoldRegion>();
    const stack: number[] = [];

    // Simple matching of `{` and `}`
    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        
        // Count `{` and `}`
        const opens = (line.match(/\{/g) || []).length;
        const closes = (line.match(/\}/g) || []).length;

        for (let o = 0; o < opens; o++) stack.push(i);
        for (let c = 0; c < closes; c++) {
            if (stack.length > 0) {
                const start = stack.pop()!;
                if (start < i) {
                    regions.set(start + 1, { start, end: i });
                }
            }
        }
    }
    
    return regions;
};

export const filterValidRegions = (regions: Map<number, FoldRegion>, foldedLines: Set<number>): Map<number, FoldRegion> => {
    // Only keep regions that are explicitly folded
    const activeRegions = new Map<number, FoldRegion>();
    for (const [line, r] of regions.entries()) {
        if (foldedLines.has(line)) {
            activeRegions.set(line, r);
        }
    }

    // Filter overlapping regions (only keep outermost)
    const validRegions = new Map<number, FoldRegion>();
    let currentEnd = -1;
    // Sort ascending by start
    const sorted = Array.from(activeRegions.entries()).sort((a,b) => a[1].start - b[1].start);
    for (const [fold, r] of sorted) {
        if (r.start > currentEnd) {
            validRegions.set(fold, r);
            currentEnd = r.end;
        }
    }
    return validRegions;
};

export const getDisplayCode = (code: string, foldedLines: Set<number>): {
    displayCode: string;
    visibleToOriginal: Record<number, number>;
} => {
    const allRegions = getFoldRegions(code);
    const regions = filterValidRegions(allRegions, foldedLines);
    
    const lines = code.split('\n');
    const out: string[] = [];
    const visibleToOriginal: Record<number, number> = {};
    
    let i = 0;
    while (i < lines.length) {
        let folded = false;
        for (const [foldId, r] of regions.entries()) {
            if (i === r.start) {
                out.push(lines[i] + ` ${FOLD_MARKER(foldId)} ` + lines[r.end].trimStart());
                visibleToOriginal[out.length - 1] = i + 1; // 1-indexed
                i = r.end + 1;
                folded = true;
                break;
            }
        }
        if (!folded) {
            out.push(lines[i]);
            visibleToOriginal[out.length - 1] = i + 1;
            i++;
        }
    }
    
    return { displayCode: out.join('\n'), visibleToOriginal };
};

export const computeActualCode = (displayCode: string, actualCode: string, foldedLines: Set<number>): string => {
    const allRegions = getFoldRegions(actualCode);
    const regions = filterValidRegions(allRegions, foldedLines);
    
    let newCode = displayCode;
    const lines = actualCode.split('\n');
    
    for (const [foldId, r] of regions.entries()) {
        const marker = FOLD_MARKER(foldId);
        
        // Match around the marker preserving exact whitespace
        const escapedMarker = marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
        const regex = new RegExp(`([\\s\\S]*?)` + escapedMarker + `([\\s\\S]*)`);
        const match = newCode.match(regex);
        
        if (match) {
            const prefix = match[1];
            const suffix = match[2];
            const internalLines = lines.slice(r.start + 1, r.end);
            
            // To perfectly reconstruct, we need to handle the space added before the marker and any space matching prefix formatting inside Editor.
            // But doing an exact reconstruction based on `displayCode` is easiest:
            // Since `out.push(lines[i] + ` ${FOLD_MARKER} ` + lines[r.end].trimStart())`
            // The user might have typed things.
            // prefix contains `lines[i] + " "`.
            const cleanPrefix = prefix.replace(/\s+$/, ""); // Remove trailing spaces from prefix
            const originalEndLineIndentation = lines[r.end].match(/^\s*/)?.[0] || '';
            const cleanSuffix = originalEndLineIndentation + suffix.trimStart();
            
            const replacement = cleanPrefix + '\n' + internalLines.join('\n') + '\n' + cleanSuffix;
            newCode = newCode.replace(regex, replacement);
        }
    }
    return newCode;
};
