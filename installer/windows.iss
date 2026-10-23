; Per-user isolated preview. Does not modify Ableton or system plugin folders.
#ifndef AppVersion
  #define AppVersion "0.1.0"
#endif
[Setup]
AppId={{E45CB310-7AAF-4D78-9093-FAC036548235}
AppName=Wave FM-1 Preview
AppVersion={#AppVersion}
AppPublisher=Wave-FM-1-plugin contributors
AppSupportURL=https://github.com/jguillenconesa-coder/Wave-FM-1-plugin
AppComments=Experimental FM-1 emulator instrument; independent of Ableton and M-VAVE.
DefaultDirName={localappdata}\Wave FM-1 Preview
DisableDirPage=yes
DisableProgramGroupPage=yes
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
#ifdef Native64Setup
SetupArchitecture=x64
#endif
MinVersion=10.0.17763
PrivilegesRequired=lowest
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
Source: "..\dist\VST3\FM-1.vst3\*"; DestDir: "{app}\VST3\FM-1.vst3"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "..\dist\licenses\*"; DestDir: "{app}\licenses"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "..\dist\INSTALL.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\dist\build-info.json"; DestDir: "{app}"; Flags: ignoreversion
