; KAI-OS Windows Installer (Inno Setup 6)
; Build: ISCC /DVersion=X.Y.Z KAI-OS.iss  (wird durch kai-os-release.yml aufgerufen)
; Es wird ausschliesslich die VORHER signierte kai-os-node.exe eingebettet.

#ifndef Version
  #error "Version fehlt: ISCC /DVersion=X.Y.Z packaging/windows/KAI-OS.iss"
#endif

#define AppName "KAI-OS Node"
#define AppExeName "kai-os-node.exe"
#define AppGuid "8f3a1c9e-52b4-4e6d-9a08-3b7c5d2e9f01"
; Hinweis: Inno-Setup-AppId nach erstem Release nicht mehr aendern (Upgrade-Erkennung).

[Setup]
AppId={{#AppGuid}
AppName={#AppName}
AppVersion={#Version}
AppVerName={#AppName} {#Version}
AppPublisher=A-TownChain-Okosystems
AppPublisherURL=https://github.com/A-TownChain-Okosystems/a-townchain-os
DefaultDirName={autopf}\KAI-OS
DefaultGroupName=KAI-OS
LicenseFile=..\..\LICENSE
Output=release
OutputBaseFilename=KAI-OS-Setup-{#Version}
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
ArchitecturesInstallIn64BitMode=x64compatible
PrivilegesRequired=admin
UninstallDisplayName={#AppName} {#Version}
MinVersion=10.0

[Files]
Source: "..\..\kai-os\target\release\kai-os-node.exe"; DestDir: "{app}\bin"; Flags: ignoreversion; Check: FileExistsSignedCheck
Source: "..\..\LICENSE"; DestDir: "{app}"; DestName: "LICENSE"; Flags: ignoreversion

[Icons]
Name: "{group}\{#AppName}"; Filename: "{app}\bin\{#AppExeName}"
Name: "{group}\Uninstall {#AppName}"; Filename: "{uninstallexe}"

[UninstallDelete]
Type: filesandordirs; Name: "{app}\bin"

[Messages]
WelcomeLabel2=Dies installiert {#AppName} {#Version} (Windows x64, signiert).

[Code]
function FileExistsSignedCheck(): Boolean;
begin
  // Fail-Closed: Installer bricht ab, wenn die signierte Binary nicht vorliegt.
  if not FileExists(ExpandConstant('..\..\kai-os\target\release\kai-os-node.exe')) then
  begin
    MsgBox('kai-os-node.exe fehlt im Release-Build. Kein Build ohne Binary.', mbError, MB_OK);
    Result := False;
    exit;
  end;
  Result := True;
end;
