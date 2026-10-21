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
        output = subprocess.CompletedProcess([], 0, "Local ADP report: " + str(self.report) + "\n", "")
        with mock.patch.object(launch, "find_executable", return_value=(self.binary, "")):
            with mock.patch.object(launch, "browser_capable", return_value=True):
                with mock.patch.object(subprocess, "run", return_value=output):
                    answer, code = launch.launch(self.project, None, False, 10)
        self.assertEqual(code, 0)
        self.assertEqual(answer["status"], "BROWSER_OPEN_REQUESTED")

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
