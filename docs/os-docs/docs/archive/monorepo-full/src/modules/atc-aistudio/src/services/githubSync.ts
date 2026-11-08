// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
export interface GithubAuditMetrics {
  pass: number;
  warn: number;
  fail: number;
}

export const fetchGithubAuditMetrics = async (repo: string): Promise<GithubAuditMetrics | null> => {
  try {
    const [issuesRes, milestonesRes] = await Promise.all([
      fetch(`https://api.github.com/repos/${repo}/issues?state=all&per_page=100`),
      fetch(`https://api.github.com/repos/${repo}/milestones?state=open`)
    ]);

    if (!issuesRes.ok || !milestonesRes.ok) {
      return null;
    }

    const issues = await issuesRes.json();
    const milestones = await milestonesRes.json();

    const openIssuesCount = issues.filter((i: any) => i.state === 'open' && !i.pull_request).length;
    const closedIssuesCount = issues.filter((i: any) => i.state === 'closed' && !i.pull_request).length;
    const openMilestonesCount = milestones.length || 0;

    // Map GitHub metrics to Audit Status counts
    // Pass = Closed Issues, Warn = Open Milestones, Fail = Open Issues
    return {
      pass: closedIssuesCount,
      warn: openMilestonesCount > 0 ? openMilestonesCount : 0,
      fail: openIssuesCount
    };
  } catch (err) {
    console.error("Github sync failed:", err);
    return null;
  }
};
