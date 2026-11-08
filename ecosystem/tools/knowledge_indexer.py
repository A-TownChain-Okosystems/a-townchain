#!/usr/bin/env python3
from __future__ import annotations
import hashlib, json, os, re, subprocess, urllib.parse, urllib.request
from datetime import datetime, timezone
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
KMS=ROOT/"docs/knowledge-system"; GENERATED=KMS/"records/generated"; REGISTRY=KMS/"registry.json"
REPO=os.environ.get("GITHUB_REPOSITORY","A-TownChain-Okosystems/a-townchain-ecosystem"); TOKEN=os.environ.get("GITHUB_TOKEN","")
NOW=datetime.now(timezone.utc).isoformat().replace("+00:00","Z"); API="https://api.github.com"
def gh(path, params=None):
    q=("?"+urllib.parse.urlencode(params)) if params else ""; h={"Accept":"application/vnd.github+json","X-GitHub-Api-Version":"2022-11-28"}
    if TOKEN: h["Authorization"]="Bearer "+TOKEN
    with urllib.request.urlopen(urllib.request.Request(API+path+q,headers=h),timeout=30) as r: return json.load(r)
def page(path, params=None, limit=100):
    p=dict(params or {}); p["per_page"]=100; out=[]
    for n in range(1,10):
        p["page"]=n; batch=gh(path,p)
        if not batch: break
        out.extend(batch)
        if len(batch)<100 or len(out)>=limit: break
    return out[:limit]
def slug(s):
    normalized=re.sub(r"[^A-Za-z0-9]+","-",s.upper()).strip("-") or "UNNAMED"
    digest=hashlib.sha256(s.encode("utf-8")).hexdigest()[:10].upper()
    return normalized[:60]+"-"+digest
def rec(rid,cat,title,ctx,stype,sref,content,tags,links=None,status="active"):
    return {"id":rid,"timestamp":NOW,"category":cat,"title":title,"context":ctx,"priority":"P2","status":status,"version":"v1.0.0","source":{"type":stype,"ref":sref,"commit":os.environ.get("GITHUB_SHA","")},"tags":sorted(set(tags)),"links":links or [],"content":content.strip()}
def main():
    GENERATED.mkdir(parents=True,exist_ok=True)
    try: files=subprocess.check_output(["git","ls-tree","-r","--name-only","HEAD"],cwd=ROOT,text=True).splitlines()
    except Exception: files=[]
    records=[]; top=sorted({p.split("/")[0] for p in files if p}); comps=sorted({p.split("/")[1] for p in files if p.startswith("components/") and len(p.split("/"))>1})
    records.append(rec("ATC-KMS-REPOSITORY-STRUCTURE","architecture","Repository structure: "+REPO,"Automatically indexed repository structure.","github","https://github.com/"+REPO,"Top-level entries: "+", ".join(top)+"\n\nComponents: "+", ".join(comps),["repository","structure","generated"],[{"relation":"contains","target":"ATC-KMS-COMPONENT-"+slug(c)} for c in comps]))
    for c in comps:
        paths=[p for p in files if p.startswith("components/"+c+"/")]; manifests=[p for p in paths if Path(p).name in {"Cargo.toml","package.json","pyproject.toml"}]
        records.append(rec("ATC-KMS-COMPONENT-"+slug(c),"module","Component: "+c,"Component subtree indexed from "+REPO+".","file","components/"+c,"Files: "+str(len(paths))+"\n\nManifests: "+(", ".join(manifests) or "none"),["component",c,"generated"],[{"relation":"part-of","target":"ATC-KMS-REPOSITORY-STRUCTURE"}]))
    for p in files:
        if Path(p).name in {"ARCHITECTURE.md","CHANGELOG.md","MIGRATION_MANIFEST.yaml","Cargo.toml","SECURITY.md","CONTRIBUTING.md"} or "architecture" in Path(p).name.lower():
            records.append(rec("ATC-KMS-SOURCE-"+slug(p),"evidence","Indexed source: "+p,"Repository architecture, release, security, migration, or build source.","file",p,"Source file present at "+p+".",["source","generated"]))
    for pr in page("/repos/"+REPO+"/pulls",{"state":"all","sort":"updated","direction":"desc"}):
        n=pr["number"]; merged=bool(pr.get("merged_at")); status="completed" if merged else ("archived" if pr["state"]=="closed" else "active")
        body=(pr.get("body") or "").strip(); content="State: "+pr["state"]+"\nMerged: "+str(merged)+"\nDraft: "+str(bool(pr.get("draft")))+"\n\n"+body
        records.append(rec("ATC-KMS-PR-"+str(n),"change","PR #"+str(n)+": "+pr["title"],"Pull request "+str(n)+" in "+REPO+".","pr",pr["html_url"],content,["github","pull-request","generated"],status=status))
    for issue in page("/repos/"+REPO+"/issues",{"state":"all","sort":"updated","direction":"desc"}):
        if "pull_request" in issue: continue
        n=issue["number"]; status="active" if issue["state"]=="open" else "archived"; content="State: "+issue["state"]+"\n\n"+(issue.get("body") or "").strip()
        records.append(rec("ATC-KMS-ISSUE-"+str(n),"discussion","Issue #"+str(n)+": "+issue["title"],"Issue "+str(n)+" in "+REPO+".","issue",issue["html_url"],content,["github","issue","generated"],status=status))
    for c in page("/repos/"+REPO+"/commits",{"sha":"main"}):
        sha=c["sha"]; title=(c.get("commit",{}).get("message") or sha).splitlines()[0]
        records.append(rec("ATC-KMS-COMMIT-"+sha[:12].upper(),"change","Commit: "+title,"Git commit "+sha+" on "+REPO+".","commit",c["html_url"],c.get("commit",{}).get("message",""),["git","commit","generated"]))
    ids={r["id"] for r in records}
    for r in records:
        for a,b in re.findall(r"(?:PR|pr)\s*#(\d+)|(?:issue|Issue)\s*#(\d+)",r["content"]):
            target="ATC-KMS-PR-"+(a or b) if a else "ATC-KMS-ISSUE-"+b
            if target in ids: r["links"].append({"relation":"references","target":target})
    for old in GENERATED.glob("*.json"): old.unlink()
    for r in sorted(records,key=lambda x:x["id"]): (GENERATED/(r["id"]+".json")).write_text(json.dumps(r,indent=2,ensure_ascii=False)+"\n")
    registry=json.loads(REGISTRY.read_text()); existing=[x for x in registry.get("records",[]) if x.get("managed_by")!="atc-kms-indexer"]
    registry["records"]=existing+[{"id":r["id"],"path":"records/generated/"+r["id"]+".json","managed_by":"atc-kms-indexer"} for r in sorted(records,key=lambda x:x["id"])]
    registry["last_updated"]=NOW; registry["indexer"]={"name":"atc-kms-indexer","version":"v1.0.0","generated_record_count":len(records)}
    REGISTRY.write_text(json.dumps(registry,indent=2,ensure_ascii=False)+"\n"); print("ATC-KMS indexed",len(records),"records from",REPO)
if __name__=="__main__": main()