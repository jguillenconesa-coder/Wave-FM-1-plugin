"""Publication must never retry away a qualified audio failure or overload."""
import contextlib
import importlib.util
import io
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location('realtime_gate', Path(__file__).resolve().parents[1] / 'tools/check-realtime.py')
gate = importlib.util.module_from_spec(spec)
spec.loader.exec_module(gate)


def output(missing=0, late=1000, speed=1.7, callback=100, halted='false', peak=0.4):
    line = (f'REALTIME X0X 48k/512 lead=1495 legacy=false polling=false: startup_missing=8000, '
            f'steady_missing={missing}, debt=0, callback_max_us={callback}, '
            f'host_late_max_us=50000, steady_host_late_max_us={late}, engine_speed={speed}, peak={peak}, halted={halted}\n')
    return line + ('steady playback missed frames\ntest result: FAILED. 0 passed; 1 failed;\n' if missing
                   else 'test result: ok. 1 passed; 0 failed;\n')


class RealtimeGateTests(unittest.TestCase):
    def test_strict_success_requires_actual_metrics_and_one_test(self):
        self.assertTrue(gate.passed(output(), 0))
        self.assertFalse(gate.passed('test result: ok. 0 passed; 0 failed;', 0))
        self.assertFalse(gate.passed(output(missing=1), 0))
        self.assertFalse(gate.passed(output(halted='true'), 0))
        self.assertFalse(gate.passed(output(speed=0.9), 0))
        self.assertFalse(gate.passed(output(callback=20000), 0))

    def test_only_failed_host_deadlines_allow_retry(self):
        self.assertTrue(gate.host_may_retry(output(missing=12000, late=49356), 101))
        self.assertFalse(gate.host_may_retry(output(missing=23646, late=5810, speed=1.331), 101))
        self.assertFalse(gate.host_may_retry(output(missing=1, late=gate.PERIOD_US), 101))

    def test_engine_overload_halts_and_callback_overruns_never_retry(self):
        for values in [{'speed':1.0}, {'speed':0.9}, {'halted':'true'}, {'callback':20000}, {'peak':0.0}]:
            self.assertFalse(gate.host_may_retry(output(missing=1000, late=50000, **values), 101))

    def test_missing_invalid_or_ambiguous_metrics_never_retry(self):
        for text in ['', output(speed=float('nan')), output(late=float('inf')),
                     output(missing=10, late=50000) * 2, 'process crashed']:
            self.assertFalse(gate.host_may_retry(text, 101))

    def exercise(self, results):
        with tempfile.TemporaryDirectory() as directory, contextlib.redirect_stdout(io.StringIO()):
            report = Path(directory) / 'report.txt'
            with patch.object(gate.subprocess, 'run', side_effect=results) as mocked:
                code = gate.run(['cargo', 'test', '--locked'], report)
            return code, mocked.call_count, report.read_text()

    def test_qualified_failure_blocks_immediately(self):
        code, calls, text = self.exercise([SimpleNamespace(returncode=101, stdout=output(missing=100, late=2000))])
        self.assertEqual((code,calls),(101,1))
        self.assertIn('publication remains blocked', text)

    def test_host_retry_retains_failure_and_requires_strict_success(self):
        code, calls, text = self.exercise([SimpleNamespace(returncode=101, stdout=output(missing=100, late=40000)),
                                         SimpleNamespace(returncode=0, stdout=output())])
        self.assertEqual((code,calls),(0,2))
        self.assertIn('steady_missing=100', text)
        self.assertIn('FINAL_REALTIME_RESULT: pass (attempt 2/3)', text)

    def test_retry_budget_is_finite(self):
        code, calls, text = self.exercise([SimpleNamespace(returncode=101, stdout=output(missing=100, late=40000))] * 3)
        self.assertEqual((code,calls),(101,3))
        self.assertNotIn('FINAL_REALTIME_RESULT: pass', text)

    def test_zero_tests_cannot_publish(self):
        code, calls, text = self.exercise([SimpleNamespace(returncode=0, stdout='test result: ok. 0 passed; 0 failed;')])
        self.assertEqual((code,calls),(1,1))
        self.assertIn('FINAL_REALTIME_RESULT: fail', text)
