# SignPath Authenticode-Signierung (fail-closed)
# Aufruf: sign-signpath.ps1 -ArtifactPath <file> -OutputPath <file>
# Konfiguration (Repository Settings -> Secrets/Variables, siehe RELEASE-SIGNING.md):
#   Secret:    SIGNPATH_API_TOKEN
#   Variablen: SIGNPATH_ORGANIZATION_ID, SIGNPATH_PROJECT_SLUG, SIGNPATH_SIGNING_POLICY_SLUG
# SignPath REST API: https://docserver.signpath.org/Web/SignPath API (v1)
param(
  [Parameter(Mandatory=$true)][string]$ArtifactPath,
  [Parameter(Mandatory=$true)][string]$OutputPath
)
$ErrorActionPreference = 'Stop'
$org     = ${env:SIGNPATH_ORGANIZATION_ID}
$project = ${env:SIGNPATH_PROJECT_SLUG}
$policy  = ${env:SIGNPATH_SIGNING_POLICY_SLUG}
if ([string]::IsNullOrWhiteSpace($env:SIGNPATH_API_TOKEN) -or
    [string]::IsNullOrWhiteSpace($org) -or
    [string]::IsNullOrWhiteSpace($project) -or
    [string]::IsNullOrWhiteSpace($policy)) {
  Write-Error @"
SignPath ist nicht konfiguriert — Fail-Closed, keine unsignierte Auslieferung.
Einzurichten (siehe packaging/windows/RELEASE-SIGNING.md):
  Secret:    SIGNPATH_API_TOKEN
  Variablen: SIGNPATH_ORGANIZATION_ID, SIGNPATH_PROJECT_SLUG, SIGNPATH_SIGNING_POLICY_SLUG
"@
  exit 1
}
if (-not (Test-Path $ArtifactPath)) { Write-Error "Artefakt fehlt: $ArtifactPath"; exit 1 }
$auth = @{ Authorization = "Bearer $env:SIGNPATH_API_TOKEN" }
$base = "https://app.signpath.org/api/v1/$org"

# 1) Signing-Request mit Artefakt anlegen (multipart)
$form = @{ projectSlug = $project; signingPolicySlug = $policy }
$resp = Invoke-RestMethod -Method Post -Uri "$base/signing-requests" -Headers $auth -Form $form
if (-not $resp -or -not $resp.id) { Write-Error "Keine Signing-Request-ID erhalten."; exit 1 }
$id = $resp.id
Write-Host "SignPath Signing-Request: $id"

# 2) Auf Abschluss warten (max. 30 Min)
$deadline = (Get-Date).AddMinutes(30)
do {
  Start-Sleep -Seconds 10
  $sr = Invoke-RestMethod -Method Get -Uri "$base/signing-requests/$id" -Headers $auth
  if ($sr.status -in @('Failed','Canceled','TimedOut')) { Write-Error "SignPath-Fehlschlag: $($sr.status)"; exit 1 }
} while ($sr.status -ne 'Completed' -and (Get-Date) -lt $deadline)
if ($sr.status -ne 'Completed') { Write-Error "Timeout: Signierung nicht in 30 Min abgeschlossen."; exit 1 }

# 3) Signiertes Artefakt herunterladen
Invoke-WebRequest -Uri "$base/signing-requests/$id/signed-artifact" -Headers $auth -OutFile $OutputPath
Write-Host "Signiert: $OutputPath"
