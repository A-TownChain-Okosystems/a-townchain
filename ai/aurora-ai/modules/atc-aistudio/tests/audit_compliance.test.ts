// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import { describe, it, expect } from 'vitest';
import { getStatusCounts, calculateAverageScore } from '../src/utils/auditUtils';

describe('Compliance Logic Tests (SoftwareAuditView)', () => {
  describe('getStatusCounts', () => {
    it('should correctly count all statuses (Pass, Warn, Fail)', () => {
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

    it('should handle empty arrays without errors', () => {
      const counts = getStatusCounts([]);
      expect(counts.Pass).toBe(0);
      expect(counts.Warn).toBe(0);
      expect(counts.Fail).toBe(0);
    });

    it('should count only existing statuses correctly', () => {
      const mockAudits = [
        { result: 'Pass', score: 90 },
      ];
      
      const counts = getStatusCounts(mockAudits);
      expect(counts.Pass).toBe(1);
      expect(counts.Warn).toBe(0);
      expect(counts.Fail).toBe(0);
    });
  });

  describe('calculateAverageScore', () => {
    it('should calculate the correct average score for standard inputs', () => {
      const mockAudits = [
        { result: 'Pass', score: 100 },
        { result: 'Pass', score: 50 },
        { result: 'Pass', score: 0 }
      ];
      
      const avg = calculateAverageScore(mockAudits);
      expect(avg).toBe(50);
    });

    it('should calculate properly rounded averages (round to nearest integer)', () => {
      const mockAudits = [
        { result: 'Pass', score: 33 },
        { result: 'Pass', score: 33 },
        { result: 'Pass', score: 35 } // sum = 101, avg = 33.666... expects 34
      ];
      
      const avg = calculateAverageScore(mockAudits);
      expect(avg).toBe(34);
    });

    it('should handle empty arrays by returning 0', () => {
      expect(calculateAverageScore([])).toBe(0);
    });
    
    it('should handle zero scores gracefully', () => {
      const mockAudits = [
        { result: 'Fail', score: 0 },
        { result: 'Fail', score: 0 },
        { result: 'Fail', score: 0 },
      ];
      const avg = calculateAverageScore(mockAudits);
      expect(avg).toBe(0);
    });
  });
});
