#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
AD-017 Modul-Sync: Produkt-Repos -> Monorepo (src/modules/)
===========================================================

Verbindliche Synchronisationsregel (AD-017, 06.09.2026):

- Kanonische Entwicklungsquelle fuer Modul-Code ist das jeweilige
  Produkt-Repo (atc-shivacore, atclang, a-townchain, globus-os,
  aurora-ai, genesis-engine).
- Der Monorepo (a-townchain-os) ist ausschliesslich INTEGRATIONSZIEL:
  Cargo-Workspace, Launch-Stack, Docker, CI, Gesamttests.
- Modul-Code wird NIE mehr direkt im Monorepo bearbeitet - nur noch
  ueber dieses Skript eingespielt (Richtung: strikt einseitig).
- Monorepo-seitige Integrationsdateien (Workspace, docker/, CI,
  Meta-Files) bleiben unberuehrt.

Modi:
  --check   Nur Bericht: Identisch / Divergent / Neu pro Repo
  --sync    Divergente + neue Dateien einspielen (Produkt-Stand gewinnt)

Das Skript laedt die Produkt-Repos als Tarball (oeffentlich, kein Token
noetig) nach .sync-cache/ und spiegelt deren modules/* nach src/modules/*.
Dateien, die nur im Monorepo existieren, werden NIE geloescht.
"""
import argparse
import hashlib
import os
import shutil
import tarfile
import urllib.request

ORG = 'A-TownChain-Okosystems'
BRANCH = 'main'
PRODUCT_REPOS = ['atc-shivacore', 'atclang', 'a-townchain',
                 'globus-os', 'aurora-ai', 'genesis-engine']
HERE = os.path.dirname(os.path.abspath(__file__))
MONO = os.path.normpath(os.path.join(HERE, '..'))
MODULES = os.path.join(MONO, 'src', 'modules')
CACHE = os.path.join(MONO, '.sync-cache')


def sha256(path):
    h = hashlib.sha256()
    with open(path, 'rb') as f:
        for chunk in iter(lambda: f.read(65536), b''):
            h.update(chunk)
    return h.hexdigest()


def fetch(repo):
    os.makedirs(CACHE, exist_ok=True)
    url = 'https://codeload.github.com/{}/{}/tar.gz/refs/heads/{}'.format(ORG, repo, BRANCH)
    tgz = os.path.join(CACHE, repo + '.tar.gz')
    with urllib.request.urlopen(url) as r:
        open(tgz, 'wb').write(r.read())
    root = os.path.join(CACHE, repo)
    if os.path.isdir(root):
        shutil.rmtree(root)
    os.makedirs(root)
    tarfile.open(tgz).extractall(root)
    return os.path.join(root, repo + '-' + BRANCH)


def walk_files(root):
    out = []
    for d, dirs, files in os.walk(root):
        dirs[:] = [x for x in dirs if x not in ('.git', 'target', '__pycache__')]
        for f in files:
            out.append(os.path.relpath(os.path.join(d, f), root))
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--check', action='store_true')
    ap.add_argument('--sync', action='store_true')
    args = ap.parse_args()
    if not args.check and not args.sync:
        args.check = True
    tot_id = tot_dv = tot_new = 0
    for repo in PRODUCT_REPOS:
        src = fetch(repo)
        modroot = os.path.join(src, 'modules')
        if not os.path.isdir(modroot):
            print('{:14s} KEIN modules/ - uebersprungen'.format(repo))
            continue
        idn = dv = new = 0
        details = []
        for mod in sorted(os.listdir(modroot)):
            pmod = os.path.join(modroot, mod)
            tmod = os.path.join(MODULES, mod)
            for rel in walk_files(pmod):
                p = os.path.join(pmod, rel)
                t = os.path.join(tmod, rel)
                if os.path.isfile(t):
                    if sha256(p) == sha256(t):
                        idn += 1
                    else:
                        dv += 1
                        details.append('DIV ' + os.path.join(mod, rel))
                        if args.sync:
                            shutil.copy2(p, t)
                else:
                    new += 1
                    details.append('NEU ' + os.path.join(mod, rel))
                    if args.sync:
                        os.makedirs(os.path.dirname(t), exist_ok=True)
                        shutil.copy2(p, t)
        tot_id += idn
        tot_dv += dv
        tot_new += new
        print('{:14s} identisch: {:4d} | divergent: {:3d} | neu: {:3d}'.format(repo, idn, dv, new))
        for d in details[:5]:
            print('    - ' + d)
        if len(details) > 5:
            print('    ... +{} weitere'.format(len(details) - 5))
    print('-' * 60)
    print('GESAMT identisch: {} | divergent: {} | neu: {}'.format(tot_id, tot_dv, tot_new))
    if args.sync and (tot_dv + tot_new) > 0:
        print('=> Synchronisiert. Jetzt im Monorepo committen:')
        print('   git add src/modules && git commit -m "sync(AD-017): Modul-Sync von Produkt-Repos"')
    if args.check and (tot_dv + tot_new) > 0:
        print('=> DIVERGENZ ERKANNT - mit --sync einspielen (Produkt-Repo gewinnt).')


if __name__ == '__main__':
    main()
