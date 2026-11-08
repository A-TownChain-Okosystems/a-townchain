// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import { describe, it, expect } from 'vitest';
import { getStatusCounts, calculateAverageScore } from './auditUtils';

describe('Audit Utility Functions', () => {
  describe('getStatusCounts', () => {
    it('should correctly count all statuses', () => {
      const mockAudits = [
        { result: 'Pass', score: 90 },
        { result: 'Pass', score: 95 },
        { result: 'Warn', score: 70 },
        { result: 'Fail', score: 40 },
        { result: 'Fail', score: 30 }
      ];
      
      const counts = getStatusCounts(mockAudits);
      expect(counts.Pass).toBe(2);
      expect(counts.Warn).toBe(1);
      expect(counts.Fail).toBe(2);
    });

    it('should handle empty arrays', () => {
      const counts = getStatusCounts([]);
      expect(counts.Pass).toBe(0);
      expect(counts.Warn).toBe(0);
      expect(counts.Fail).toBe(0);
    });
  });

  describe('calculateAverageScore', () => {
    it('should calculate the correct average score', () => {
      const mockAudits = [
        { result: 'Pass', score: 100 },
        { result: 'Pass', score: 50 }
      ];
      
      const avg = calculateAverageScore(mockAudits);
      expect(avg).toBe(75);
    });

    it('should round the average to nearest integer', () => {
      const mockAudits = [
        { result: 'Pass', score: 33 },
        { result: 'Pass', score: 33 },
        { result: 'Pass', score: 35 } // sum = 101, avg = 33.666
      ];
      
      const avg = calculateAverageScore(mockAudits);
      expect(avg).toBe(34);
    });

    it('should return 0 for empty arrays', () => {
      expect(calculateAverageScore([])).toBe(0);
    });
  });
});
