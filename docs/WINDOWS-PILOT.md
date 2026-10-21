# Windows desktop pilot — browser opening and project non-interference

**Purpose:** verify the real user experience from an interactive Windows desktop, including coding-agent hosts such as Antigravity. A green GitHub Actions Windows build does **not** establish that a browser tab appeared on the user's screen.

This is a human/local-agent testing procedure, **not** permission to update application files, work records, `.gitignore`, Git history, or project dependencies. The Observatory executable and skill remain optional; no protocol changes are required.

## Test the latest code — safely

1. Identify the **existing** trusted Observatory source checkout or already-reviewed executable **outside** the target project. Do not clone a second copy or install a second executable just to run this test.
2. If you need the browser fix, update the existing Observatory source checkout to a reviewed exact commit by your normal authorized Git process, ensure it has no unrelated uncommitted changes, and rebuild with `cargo build --release`. Alternatively, verify a supported GitHub Actions preview binary's provenance and use its absolute path. Neither source compilation nor unverified download is a routine prerequisite for users.
3. During diagnosis, invoke the **newly built or verified executable by its explicit absolute path**. Do not assume `adp-observatory` on PATH points to the same version: older copies can exist in `.cargo\\bin` and `%LOCALAPPDATA%\\Programs\\ADP-Observatory`.
4. Run from an interactive PowerShell session. Replace the sample executable and project directories with the real **approved** absolute paths:

   ```powershell
   $exe = 'C:\path\to\reviewed\ADP-Observatory\target\release\adp-observatory.exe'
   $project = 'C:\path\to\your\ADP-project'

   # Generate only one report and do not attempt browser opening:
   & $exe dashboard $project --no-open
   ```

   Confirm that output contains `Local ADP report: C:\...html` without the Windows extended path prefix, as well as `Browser launch: NOT_REQUESTED`. This output is not an error. The report should be outside `$project`.

5. Open **that existing file** manually in the same desktop session. Copy the exact `Local ADP report:` path from the previous command:

   ```powershell
   $report = 'C:\path\from\Local ADP report\report.html'
   Test-Path -LiteralPath $report
   Invoke-Item -LiteralPath $report
   ```

   If this fails, review Windows' default `.html` association, session permissions and IDE desktop integration before blaming the Observatory renderer. Do not bypass OS security warnings.

6. To test the automatic ShellExecuteW opener, make **one** additional explicit invocation:

   ```powershell
   & $exe dashboard $project
   ```

   Record the exact `Browser launch:` line:
   - `REQUESTED (visibility not confirmed)`: the OS accepted the request; **ask the user whether the report was visible**. IDE terminal/headless sessions can still have no visible browser window.
   - `FAILED`: the opener reported a failure. The HTML is still saved; use the path from output and show the failure message.
   - `NOT_REQUESTED`: you passed `--no-open` or used headless mode. Do not claim the browser opened.
   - **No marker**: likely an older executable. Do not claim success; use the returned report path and identify the binary you launched.

7. If using the optional Python Agent Skill launcher, inspect its `executable` and `alternate_installations` JSON fields. The first is the binary actually selected; any alternates are **informational only**, and the launcher does not execute, update or delete them. This helps identify version drift when both the application directory and `.cargo\\bin` contain copies.
8. Once the result is understood, decide whether to replace the existing installed executable via the project's approved install/update procedure. **Do not silently copy new binaries into both installation locations** or change PATH/IDE settings just to make tests appear successful.

## Agent-level non-interference acceptance

Before and after setup or visualization, an agent may **read** Git status if permitted and compare it. It must not make any project changes:

- Do **not** create or regenerate optional `.project/WORK.json` just because the dashboard is missing contracts.
- Do **not** execute the analyzed project's test/build scripts, stage/commit/push its files, change `.gitignore`, or produce project-local artifact copies as part of visualization.
- Missing optional records must show as **unavailable/not assessed**, not be fabricated.
- Do **not** repeatedly generate new cached reports or duplicate them into IDE storage to compensate for a browser-launch failure.
- Return the actual report file path, executable path/version, and browser-launch marker. A successful exit code means a report was generated; it does not prove a browser opened.

## Pilot result format

```text
Observatory build/source: <reviewed commit or verified release>
Executable: <absolute location>
Analyzed project: <path>
Report generated outside project: YES / NO
Browser launch result: REQUESTED / FAILED / NOT_REQUESTED / UNKNOWN
Report actually visible in user's browser: YES / NO / NOT CONFIRMED
Project Git/files unchanged: YES / NO / NOT CHECKED
Optional WORK.json created or changed by setup: NO (required)
External project scripts executed: NO (required)
Additional notes: <errors, IDE host/desktop behavior>
```

**Shipping gate:** tests on real interactive Windows desktop sessions and the actual agent host remain necessary. Automated CI covers compilation, Windows path conversion, status semantics, generated HTML and no project-file writes, not user-visible browser focus.
