// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
export interface PartialAuditData {
  result: string;
  score: number;
}

export const getStatusCounts = (audits: PartialAuditData[]) => {
  const result = {
    Pass: 0,
    Warn: 0,
    Fail: 0
  };
  
  audits.forEach(audit => {
    if (audit.result === 'Pass') result.Pass++;
    if (audit.result === 'Warn') result.Warn++;
    if (audit.result === 'Fail') result.Fail++;
  });
  
  return result;
};

export const calculateAverageScore = (audits: PartialAuditData[]) => {
  if (!audits || audits.length === 0) return 0;
  const totalScore = audits.reduce((acc, curr) => acc + curr.score, 0);
  return Math.round(totalScore / audits.length);
};
