# Experimental Windows installer for a pinned, GitHub-attested CI build.
# Run only from a trusted local checkout. Never execute remote scripts from a pipe.
[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [ValidatePattern('^[0-9]+$')]
    [string]$RunId,

    [Parameter(Mandatory = $true)]
    [ValidatePattern('^[a-fA-F0-9]{40}$')]
    [string]$ExpectedSha,

    [string]$InstallDir = (Join-Path $env:LOCALAPPDATA 'Programs\ADP-Observatory')
)
$ErrorActionPreference = 'Stop'
$Repo = 'FrederikLive/ADP-Observatory'
$Workflow = "$Repo/.github/workflows/prototype-packages.yml"
$ExpectedSha = $ExpectedSha.ToLowerInvariant()

function Invoke-Gh([string[]]$Arguments) {
    & gh @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "GitHub CLI command failed (exit code $LASTEXITCODE)"
    }
}
function Assert-OutsideProject([string]$Candidate) {
    $current = [IO.Path]::GetFullPath((Get-Location).Path).TrimEnd('\')
    $candidatePath = [IO.Path]::GetFullPath($Candidate).TrimEnd('\')
    if ($candidatePath.Equals($current, [StringComparison]::OrdinalIgnoreCase) -or
        $candidatePath.StartsWith("$current\", [StringComparison]::OrdinalIgnoreCase)) {
        throw 'Installation must not create files inside the current ADP project'
    }
}
if ([Runtime.InteropServices.RuntimeInformation]::OSArchitecture -ne
    [Runtime.InteropServices.Architecture]::X64) {
    throw 'Only Windows x64 preview binaries are currently supported'
}
if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
    throw 'Install and authenticate the GitHub CLI (gh) before using this preview installer'
}
$runJson = (Invoke-Gh -Arguments @(
    'run','view',$RunId,'-R',$Repo,'--json','conclusion,headSha,workflowName,headBranch'
) | Out-String | ConvertFrom-Json)
if ($runJson.conclusion -ne 'success' -or
    $runJson.headSha.ToLowerInvariant() -ne $ExpectedSha -or
    $runJson.workflowName -ne 'Prototype binary packages' -or
    $runJson.headBranch -ne 'main') {
    throw 'Selected CI run is not a successful main-branch preview at the exact requested SHA'
}

$temp = Join-Path ([IO.Path]::GetTempPath()) ("adp-obs-install-" + [Guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $temp -ErrorAction Stop | Out-Null
$staged = $null
try {
    Invoke-Gh -Arguments @('run','download',$RunId,'-R',$Repo,
        '-n','adp-observatory-windows-x64-prototype','-D',$temp) | Out-Null
    $binary = Join-Path $temp 'adp-observatory.exe'
    $digestFile = Join-Path $temp 'SHA256SUMS.txt'
    if (-not (Test-Path -LiteralPath $binary -PathType Leaf) -or
        -not (Test-Path -LiteralPath $digestFile -PathType Leaf)) {
        throw 'Binary or checksum file is missing from downloaded artifact'
    }
    foreach ($path in @($binary, $digestFile)) {
        if (((Get-Item -LiteralPath $path -Force).Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) {
            throw 'Symlink or reparse-point artifact paths are not supported'
        }
    }
    $lines = Get-Content -LiteralPath $digestFile
    $checksumLines = @($lines | Where-Object { $_ -match '^([a-fA-F0-9]{64})  adp-observatory\.exe$' })
    if ($checksumLines.Count -ne 1) { throw 'Missing or duplicate SHA-256 entry' }
    $expectedDigest = $checksumLines[0].Substring(0,64).ToLowerInvariant()
    $actualDigest = (Get-FileHash -LiteralPath $binary -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($expectedDigest -ne $actualDigest) { throw 'Binary digest mismatch' }
    Invoke-Gh -Arguments @('attestation','verify',$binary,'--repo',$Repo,
        '--signer-workflow',$Workflow,'--source-digest',$ExpectedSha,
        '--source-ref','refs/heads/main') | Out-Null

    $InstallDir = [IO.Path]::GetFullPath($InstallDir)
    Assert-OutsideProject $InstallDir
    New-Item -ItemType Directory -Path $InstallDir -Force -ErrorAction Stop | Out-Null
    $folder = Get-Item -LiteralPath $InstallDir -Force
    if (($folder.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) {
        throw 'Installation directory must not be a junction or symlink'
    }
    $actualDir = (Resolve-Path -LiteralPath $InstallDir).ProviderPath
    Assert-OutsideProject $actualDir
    $destination = Join-Path $actualDir 'adp-observatory.exe'
    if (Test-Path -LiteralPath $destination) {
        throw "An installation already exists at $destination; refusing to overwrite"
    }

    $staged = Join-Path $actualDir (".adp-observatory-" + [Guid]::NewGuid().ToString('N') + '.exe')
    Copy-Item -LiteralPath $binary -Destination $staged -ErrorAction Stop
    if (Test-Path -LiteralPath $destination) { throw 'Destination appeared during installation' }
    Move-Item -LiteralPath $staged -Destination $destination -ErrorAction Stop
    $staged = $null
    & $destination --version
    if ($LASTEXITCODE -ne 0) { throw 'Installed executable failed startup smoke check' }
    Write-Host ""
    Write-Host "Installed verified preview: $destination"
    Write-Host ('Run from any ADP project using: & "' + $destination + '" .')
    Write-Host "Add $actualDir to PATH manually to use: adp-observatory ."
    Write-Host "Uninstall: delete $destination"
}
finally {
    if ($staged -and (Test-Path -LiteralPath $staged)) {
        Remove-Item -LiteralPath $staged -Force -ErrorAction SilentlyContinue
    }
    Remove-Item -LiteralPath $temp -Recurse -Force -ErrorAction SilentlyContinue
}
