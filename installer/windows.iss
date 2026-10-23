; Build with Inno Setup 6.3+ after tools/build-windows.ps1.
#ifndef AppVersion
  #define AppVersion "0.1.0"
#endif
#define AppIdValue "{E45CB310-7AAF-4D78-9093-FAC036548235}"
[Setup]
AppId={{E45CB310-7AAF-4D78-9093-FAC036548235}
AppName=Wave FM-1
AppVersion={#AppVersion}
AppPublisher=Wave-FM-1-plugin contributors
DefaultDirName={autopf}\Wave FM-1
DisableDirPage=yes
DisableProgramGroupPage=yes
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0.17763
PrivilegesRequired=admin
OutputDir=..\dist\installer
OutputBaseFilename=Wave-FM-1-{#AppVersion}-Windows-x64-Setup
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
LicenseFile=..\LICENSE
InfoBeforeFile=NOTICE.txt
UninstallDisplayName=Wave FM-1
RestartApplications=no
CloseApplications=no
SetupLogging=yes
[Languages]
Name: "spanish"; MessagesFile: "compiler:Languages\Spanish.isl"
Name: "english"; MessagesFile: "compiler:Default.isl"
[Files]
Source: "..\dist\VST3\FM-1.vst3"; DestDir: "{commoncf64}\VST3"; Flags: ignoreversion uninsneveruninstall
Source: "..\dist\CLAP\FM-1.clap"; DestDir: "{commoncf64}\CLAP"; Flags: ignoreversion uninsneveruninstall
Source: "..\dist\licenses\*"; DestDir: "{app}\licenses"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "..\dist\INSTALL.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\dist\build-info.json"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\build\prerequisites\VC_redist.x64.exe"; Flags: dontcopy
[Registry]
Root: HKLM64; Subkey: "Software\WaveFM1"; Flags: uninsdeletekey
[Code]
function MatchesInstalledFile(Path, Name: String): Boolean;
var
  Expected: String;
begin
  Result := False;
  if not FileExists(Path) then exit;
  if not RegQueryStringValue(HKLM64, 'Software\WaveFM1', Name, Expected) then exit;
  Result := (Expected <> '') and (CompareText(GetSHA256OfFile(Path), Expected) = 0);
end;

function CanReplacePlugin(Path, Name: String; Owned: Boolean): Boolean;
begin
  Result := False;
  if DirExists(Path) then exit;
  if not FileExists(Path) then
    Result := True
  else if Owned then
    Result := MatchesInstalledFile(Path, Name);
end;

function InitializeSetup(): Boolean;
var
  Owned: Boolean;
begin
  Owned := RegKeyExists(HKLM64,
    'Software\Microsoft\Windows\CurrentVersion\Uninstall\{#AppIdValue}_is1');
  Result := True;
  if (not CanReplacePlugin(ExpandConstant('{commoncf64}\VST3\FM-1.vst3'), 'VST3SHA256', Owned)) or
     (not CanReplacePlugin(ExpandConstant('{commoncf64}\CLAP\FM-1.clap'), 'CLAPSHA256', Owned)) or
     FileExists(ExpandConstant('{localappdata}\Programs\Common\CLAP\FM-1.clap')) then
  begin
    MsgBox('An existing FM-1 plugin was found. Back it up and remove that installation before installing Wave FM-1. No files have been replaced.', mbError, MB_OK);
    Result := False;
  end;
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  if CurStep = ssPostInstall then
  begin
    if not RegWriteStringValue(HKLM64, 'Software\WaveFM1', 'VST3SHA256',
        GetSHA256OfFile(ExpandConstant('{commoncf64}\VST3\FM-1.vst3'))) then
      RaiseException('Could not record VST3 installation ownership.');
    if not RegWriteStringValue(HKLM64, 'Software\WaveFM1', 'CLAPSHA256',
        GetSHA256OfFile(ExpandConstant('{commoncf64}\CLAP\FM-1.clap'))) then
      RaiseException('Could not record CLAP installation ownership.');
  end;
end;

procedure RemoveOwnedPlugin(Path, Name: String);
begin
  if MatchesInstalledFile(Path, Name) then
  begin
    if not DeleteFile(Path) then
      Log('Could not remove plugin (close audio hosts): ' + Path);
  end
  else if FileExists(Path) then
    Log('Preserved plugin replaced outside this installer: ' + Path);
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  if CurUninstallStep = usUninstall then
  begin
    RemoveOwnedPlugin(ExpandConstant('{commoncf64}\VST3\FM-1.vst3'), 'VST3SHA256');
    RemoveOwnedPlugin(ExpandConstant('{commoncf64}\CLAP\FM-1.clap'), 'CLAPSHA256');
  end;
end;

function PrepareToInstall(var NeedsRestart: Boolean): String;
var
  ExitCode: Integer;
begin
  Result := '';
  ExtractTemporaryFile('VC_redist.x64.exe');
  if not Exec(ExpandConstant('{tmp}\VC_redist.x64.exe'),
      '/install /quiet /norestart', '', SW_HIDE, ewWaitUntilTerminated, ExitCode) then
    Result := 'Could not start Microsoft Visual C++ Runtime setup.'
  else if ExitCode = 3010 then
    NeedsRestart := True
  else if (ExitCode <> 0) and (ExitCode <> 1638) then
    Result := Format('Microsoft Visual C++ Runtime setup failed (code %d).', [ExitCode]);
end;
