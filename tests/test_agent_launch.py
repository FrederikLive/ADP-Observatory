"""Offline regression tests for the optional Observatory Agent Skill launcher."""
import importlib.util
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest import mock

source = Path(__file__).resolve().parents[1] / "skills/adp-observatory/scripts/launch.py"
spec = importlib.util.spec_from_file_location("observatory_launcher", source)
launch = importlib.util.module_from_spec(spec)
spec.loader.exec_module(launch)


class AgentLaunchTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.project = Path(self.tmp.name) / "project"
        self.project.mkdir()
        self.report = Path(self.tmp.name) / "report ø.html"
        self.report.write_text("<!doctype html>", encoding="utf8")
        self.binary = Path(self.tmp.name) / "approved-executable"
        self.binary.write_text("fixture", encoding="utf8")
        self.binary.chmod(0o755)

    def test_executable_inside_project_rejected(self):
        candidate = self.project / "observatory"
        candidate.write_text("fixture", encoding="utf8")
        candidate.chmod(0o755)
        with mock.patch.object(launch, "executable_candidates", return_value=[candidate]):
            binary, _ = launch.find_executable(self.project, None)
        self.assertIsNone(binary)

    def test_relative_executable_rejected(self):
        binary, reason = launch.find_executable(self.project, "./observatory")
        self.assertIsNone(binary)
        self.assertIn("absolute", reason)

    def test_absent_executable_does_not_run_commands(self):
        with mock.patch.object(launch, "executable_candidates", return_value=[]):
            with mock.patch.object(subprocess, "run") as runner:
                answer, code = launch.launch(self.project, None, False, 10)
        runner.assert_not_called()
        self.assertEqual(code, launch.EXIT_NOT_INSTALLED)
        self.assertEqual(answer["status"], "NOT_INSTALLED")

    def test_headless_scan_generates_report(self):
        output = subprocess.CompletedProcess([], 0, "Local ADP report: " + str(self.report) + "\n", "")
        with mock.patch.object(launch, "find_executable", return_value=(self.binary, "")):
            with mock.patch.object(launch, "browser_capable", return_value=False):
                with mock.patch.object(subprocess, "run", return_value=output) as runner:
                    answer, code = launch.launch(self.project, None, False, 10)
        self.assertEqual(code, 0)
        self.assertEqual(answer["status"], "REPORT_SAVED")
        self.assertIn("--no-open", runner.call_args.args[0])

    def test_desktop_attempt_is_not_claimed_browser_visible(self):
        output = subprocess.CompletedProcess([], 0, "Local ADP report: " + str(self.report) + "\nBrowser launch: REQUESTED (visibility not confirmed)\n", "")
        with mock.patch.object(launch, "find_executable", return_value=(self.binary, "")):
            with mock.patch.object(launch, "browser_capable", return_value=True):
                with mock.patch.object(subprocess, "run", return_value=output):
                    answer, code = launch.launch(self.project, None, False, 10)
        self.assertEqual(code, 0)
        self.assertEqual(answer["status"], "BROWSER_OPEN_REQUESTED")

    def test_failed_browser_open_does_not_claim_success(self):
        output = subprocess.CompletedProcess(
            [], 0,
            "Local ADP report: " + str(self.report) + "\nBrowser launch: FAILED\n",
            "Browser could not be opened automatically: no desktop association",
        )
        with mock.patch.object(launch, "find_executable", return_value=(self.binary, "")):
            with mock.patch.object(launch, "browser_capable", return_value=True):
                with mock.patch.object(subprocess, "run", return_value=output):
                    answer, code = launch.launch(self.project, None, False, 10)
        self.assertEqual(code, 0)
        self.assertEqual(answer["status"], "REPORT_SAVED")
        self.assertEqual(answer["browser_launch_status"], "FAILED")
        self.assertTrue(answer["browser_open_attempted"])
        self.assertEqual(answer["report_path"], str(self.report))

    def test_old_executable_without_browser_marker_does_not_claim_success(self):
        output = subprocess.CompletedProcess(
            [], 0, "Local ADP report: " + str(self.report) + "\n", ""
        )
        with mock.patch.object(launch, "find_executable", return_value=(self.binary, "")):
            with mock.patch.object(launch, "browser_capable", return_value=True):
                with mock.patch.object(subprocess, "run", return_value=output):
                    answer, code = launch.launch(self.project, None, False, 10)
        self.assertEqual(code, 0)
        self.assertEqual(answer["status"], "REPORT_SAVED")
        self.assertEqual(answer["browser_launch_status"], "UNKNOWN")

    def test_launch_reports_invalid_ledger_without_extra_scans(self):
        output = subprocess.CompletedProcess(
            [], 0,
            "Local ADP report: " + str(self.report) +
            "\nWork index status: INVALID\nDelivery metric state: PARTIAL\n"
            "Work index errors: 92\nBrowser launch: REQUESTED (visibility not confirmed)\n",
            ""
        )
        with mock.patch.object(launch, "find_executable", return_value=(self.binary, "")):
            with mock.patch.object(launch, "browser_capable", return_value=True):
                with mock.patch.object(subprocess, "run", return_value=output) as runner:
                    answer, code = launch.launch(self.project, None, False, 10)
        self.assertEqual(code, 0)
        self.assertEqual(answer["work_index_status"], "INVALID")
        self.assertEqual(answer["delivery_metric_state"], "PARTIAL")
        self.assertEqual(answer["work_index_error_count"], 92)
        self.assertEqual(answer["status"], "BROWSER_OPEN_REQUESTED")
        runner.assert_called_once()

    def test_only_generates_one_report_per_launch_request(self):
        output = subprocess.CompletedProcess(
            [], 0, "Local ADP report: " + str(self.report) + "\nBrowser launch: NOT_REQUESTED\n", ""
        )
        with mock.patch.object(launch, "find_executable", return_value=(self.binary, "")):
            with mock.patch.object(subprocess, "run", return_value=output) as runner:
                reply, code = launch.launch(self.project, None, True, 10)
        self.assertEqual(code, 0)
        self.assertEqual(reply["browser_launch_status"], "NOT_REQUESTED")
        runner.assert_called_once()
        self.assertNotIn("scan", runner.call_args.args[0])

    def test_detects_duplicate_executable_without_running_it(self):
        other = Path(self.tmp.name) / "old-observatory"
        other.write_text("old copy", encoding="utf8")
        other.chmod(0o755)
        with mock.patch.object(launch, "executable_candidates", return_value=[self.binary, other]):
            found = launch.alternate_installations(self.project, self.binary.resolve(), None)
        self.assertEqual(found, [str(other.resolve())])
        with mock.patch.object(launch, "executable_candidates", return_value=[self.binary, other]):
            self.assertEqual(
                launch.alternate_installations(self.project, self.binary.resolve(), str(self.binary)),
                [],
            )

    def test_remote_host_does_not_claim_browser(self):
        for variable in ("CI", "SSH_CONNECTION", "SSH_TTY"):
            with mock.patch.dict(os.environ, {variable: "true"}):
                self.assertFalse(launch.browser_capable())

    def test_extracts_unicode_report_path(self):
        self.assertEqual(
            launch.extract_report_path("Local ADP report: /tmp/ø report.html\n"),
            "/tmp/ø report.html",
        )


if __name__ == "__main__":
    unittest.main()
