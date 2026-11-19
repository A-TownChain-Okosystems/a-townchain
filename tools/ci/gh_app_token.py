#!/usr/bin/env python3
# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""GitHub-App-Installationstoken erzeugen (SCR-0147).

Warum eine GitHub App und kein PAT: Workflow-Dateien unter .github/workflows/
lassen sich nur mit dem Scope `workflow` schreiben. Ein Personal Access Token
mit diesem Scope ist ein Dauerzugang fuer das gesamte Konto. Eine GitHub App
bekommt genau die Rechte, die sie braucht (Contents, Pull requests, Workflows)
und nur fuer die Repositories, in denen sie installiert ist. Ihr Token gilt eine
Stunde und wird hier bei Bedarf neu erzeugt.

Aufruf:
    python3 tools/ci/gh_app_token.py --out /tmp/gh_app_token   # Datei, Modus 600
    python3 tools/ci/gh_app_token.py --check                   # Rechte anzeigen

Der Token wird nie auf stdout geschrieben und nie geloggt.
"""

from __future__ import annotations

import argparse
import base64
import json
import os
import stat
import sys
import time
import urllib.error
import urllib.request

try:
    from cryptography.hazmat.primitives import hashes, serialization
    from cryptography.hazmat.primitives.asymmetric import padding
except ImportError:  # pragma: no cover
    sys.exit("cryptography fehlt: pip install cryptography")


def _b64(raw: bytes) -> str:
    return base64.urlsafe_b64encode(raw).rstrip(b"=").decode("ascii")


def make_jwt(app_id: str, private_key_pem: bytes, now: int | None = None) -> str:
    """Signiertes App-JWT (RS256), 9 Minuten gueltig, 60 s Toleranz zurueck."""
    issued = int(time.time() if now is None else now)
    header = {"alg": "RS256", "typ": "JWT"}
    payload = {"iat": issued - 60, "exp": issued + 540, "iss": app_id}
    signing_input = "{}.{}".format(
        _b64(json.dumps(header, separators=(",", ":")).encode()),
        _b64(json.dumps(payload, separators=(",", ":")).encode()),
    )
    key = serialization.load_pem_private_key(private_key_pem, password=None)
    signature = key.sign(signing_input.encode(), padding.PKCS1v15(), hashes.SHA256())
    return f"{signing_input}.{_b64(signature)}"


def _api(path: str, jwt: str, method: str = "POST") -> dict:
    request = urllib.request.Request(
        f"https://api.github.com{path}",
        method=method,
        headers={
            "Authorization": f"Bearer {jwt}",
            "Accept": "application/vnd.github+json",
            "X-GitHub-Api-Version": "2022-11-28",
            "User-Agent": "atc-gh-app-token",
        },
    )
    with urllib.request.urlopen(request, timeout=30) as response:
        return json.loads(response.read())


def installation_token(app_id: str, installation_id: str, pem: bytes) -> dict:
    return _api(f"/app/installations/{installation_id}/access_tokens", make_jwt(app_id, pem))


def _load() -> tuple[str, str, bytes]:
    app_id = os.environ.get("GITHUB_APP_ID", "").strip()
    installation_id = os.environ.get("GITHUB_APP_INSTALLATION_ID", "").strip()
    pem = os.environ.get("GITHUB_APP_PRIVATE_KEY", "")
    missing = [
        name
        for name, value in (
            ("GITHUB_APP_ID", app_id),
            ("GITHUB_APP_INSTALLATION_ID", installation_id),
            ("GITHUB_APP_PRIVATE_KEY", pem),
        )
        if not value
    ]
    if missing:
        sys.exit(f"Fehlende Umgebungsvariablen: {', '.join(missing)}")
    if "\\n" in pem and "-----BEGIN" in pem:
        pem = pem.replace("\\n", "\n")
    return app_id, installation_id, pem.encode()


def main() -> int:
    parser = argparse.ArgumentParser(description="GitHub-App-Installationstoken erzeugen")
    parser.add_argument("--out", help="Zieldatei fuer den Token (Modus 600)")
    parser.add_argument("--check", action="store_true", help="nur Rechte und Ablauf anzeigen")
    args = parser.parse_args()

    app_id, installation_id, pem = _load()
    try:
        data = installation_token(app_id, installation_id, pem)
    except urllib.error.HTTPError as error:
        detail = error.read().decode(errors="replace")[:200]
        sys.exit(f"GitHub lehnt das App-JWT ab (HTTP {error.code}): {detail}")
    except urllib.error.URLError as error:
        sys.exit(f"GitHub nicht erreichbar: {error.reason}")

    token = data["token"]
    if args.check or not args.out:
        print(f"App-ID:          {app_id}")
        print(f"Installation:    {installation_id}")
        print(f"gueltig bis:     {data.get('expires_at')}")
        print(f"Repositories:    {len(data.get('repositories', [])) or 'alle der Installation'}")
        print(f"Token-Laenge:    {len(token)} Zeichen (Wert wird nicht ausgegeben)")
        return 0

    with open(args.out, "w", encoding="utf-8") as handle:
        handle.write(token)
    os.chmod(args.out, stat.S_IRUSR | stat.S_IWUSR)
    print(f"Token geschrieben: {args.out} (600), gueltig bis {data.get('expires_at')}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
