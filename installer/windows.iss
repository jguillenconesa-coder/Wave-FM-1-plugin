; Standard VST3 installation. Only this product's bundle and uninstall key.
#ifndef AppVersion
  #define AppVersion "0.2.6"
#endif
[Setup]
AppId={{E45CB310-7AAF-4D78-9093-FAC036548235}
AppName=Wave FM-1 Preview
AppVersion={#AppVersion}
AppPublisher=Wave-FM-1-plugin contributors
AppSupportURL=https://github.com/jguillenconesa-coder/Wave-FM-1-plugin
AppComments=Experimental FM-1 emulator instrument; independent of Ableton and M-VAVE.
DefaultDirName={commoncf64}\VST3\Wave FM-1.vst3
UsePreviousAppDir=no
DisableDirPage=no
DisableProgramGroupPage=yes
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
#ifdef Native64Setup
SetupArchitecture=x64
#endif
MinVersion=10.0.17763
PrivilegesRequired=admin
Uninstallable=yes
CreateUninstallRegKey=yes
UninstallDisplayName=Wave FM-1 Preview
OutputDir=..\dist\installer
OutputBaseFilename=Wave-FM-1-{#AppVersion}-Windows-x64-Setup
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
LicenseFile=..\LICENSE
InfoBeforeFile=NOTICE.txt
RestartApplications=no
CloseApplications=no
SetupLogging=yes
[Languages]
Name: "spanish"; MessagesFile: "compiler:Languages\Spanish.isl"
Name: "english"; MessagesFile: "compiler:Default.isl"
[Files]
Source: "..\dist\VST3\FM-1.vst3\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "..\dist\licenses\*"; DestDir: "{app}\licenses"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "..\dist\INSTALL.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\dist\build-info.json"; DestDir: "{app}"; Flags: ignoreversion

[Code]
function InitializeSetup(): Boolean;
var
  OldVersion: String;
begin
  Result := True;
  if RegQueryStringValue(HKCU, 'Software\Microsoft\Windows\CurrentVersion\Uninstall\{E45CB310-7AAF-4D78-9093-FAC036548235}_is1', 'DisplayVersion', OldVersion) then
    if OldVersion = '0.1.0' then begin
      SuppressibleMsgBox('Uninstall Wave FM-1 Preview 0.1.0 from Windows Settings > Apps first, then run this installer again. Keep your projects and exported saves. This prevents two copies of the same VST3 being scanned.', mbInformation, MB_OK, IDOK);
      Result := False;
    end;
end;

function CheckDirectory(): String;
begin
  Result := '';
    if CompareText(ExtractFileName(RemoveBackslashUnlessRoot(WizardDirValue)), 'Wave FM-1.vst3') <> 0 then begin
      Result := 'Choose a folder ending in Wave FM-1.vst3. The installer must have its own plugin bundle.';
    end else if DirExists(WizardDirValue) and
      not FileExists(AddBackslash(WizardDirValue) + 'unins000.dat') then begin
      Result := 'This folder already exists and is not managed by this installer. Choose another location; no files have been replaced.';
    end;
end;

function NextButtonClick(CurPageID: Integer): Boolean;
var
  Error: String;
begin
  Result := True;
  if CurPageID = wpSelectDir then begin
    Error := CheckDirectory();
    if Error <> '' then begin
      SuppressibleMsgBox(Error, mbError, MB_OK, IDOK);
      Result := False;
    end;
  end;
end;

function PrepareToInstall(var NeedsRestart: Boolean): String;
begin
  Result := CheckDirectory();
end;
