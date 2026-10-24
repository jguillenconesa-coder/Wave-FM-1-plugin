"""Run the strict X0X diagnostic; retry only an unqualified synthetic host.

Shared CI runners cannot guarantee audio deadlines. A failed diagnostic may be
repeated at most twice if the fake host itself missed a 512/48k callback period,
while the engine still exceeded realtime and its callback remained within budget.
All attempts are retained. A qualified failure, halt, overload, missing report or
three unsuccessful attempts blocks publication. The Rust assertions stay strict.
"""
import argparse
import math
from pathlib import Path
import re
import subprocess

PERIOD_US = 512 / 48000 * 1_000_000
MAX_ATTEMPTS = 3


def metrics(output):
    lines = [line for line in output.splitlines() if line.startswith('REALTIME X0X 48k/512 ')]
    if len(lines) != 1:
        return None
    values = dict(re.findall(r'([a-z_]+)=([^,\s]+)', lines[0]))
    try:
        result = {key: float(values[key]) for key in
                  ('steady_missing', 'steady_host_late_max_us', 'callback_max_us', 'engine_speed', 'peak')}
        if not all(math.isfinite(value) and value >= 0 for value in result.values()):
            return None
        if values['halted'] not in ('true', 'false'):
            return None
        result['halted'] = values['halted'] == 'true'
        return result
    except (KeyError, ValueError):
        return None


def passed(output, code):
    data = metrics(output)
    return (code == 0 and data is not None and data['steady_missing'] == 0
            and not data['halted'] and data['peak'] > 0.0001
            and data['engine_speed'] > 1.0 and data['callback_max_us'] < PERIOD_US
            and re.search(r'test result: ok\. 1 passed; 0 failed;', output) is not None)


def host_may_retry(output, code):
    data = metrics(output)
    return (code != 0 and data is not None and data['steady_missing'] > 0
            and not data['halted'] and data['peak'] > 0.0001
            and data['engine_speed'] > 1.0
            and data['callback_max_us'] < PERIOD_US
            and data['steady_host_late_max_us'] > PERIOD_US
            and 'steady playback missed frames' in output
            and 'test result: FAILED. 0 passed; 1 failed;' in output)


def run(command, report):
    report.parent.mkdir(parents=True, exist_ok=True)
    with report.open('w', encoding='utf-8') as stream:
        def emit(value):
            print(value, flush=True)
            stream.write(value + '\n')
            stream.flush()
        for attempt in range(1, MAX_ATTEMPTS + 1):
            emit(f'=== X0X timing attempt {attempt}/{MAX_ATTEMPTS} ===')
            # Argument array, no shell and no command derived from report text.
            result = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                                    text=True, encoding='utf-8', errors='replace', check=False)
            emit(result.stdout.rstrip())
            if passed(result.stdout, result.returncode):
                emit(f'FINAL_REALTIME_RESULT: pass (attempt {attempt}/{MAX_ATTEMPTS})')
                return 0
            if attempt == MAX_ATTEMPTS or not host_may_retry(result.stdout, result.returncode):
                emit('FINAL_REALTIME_RESULT: fail; publication remains blocked')
                return result.returncode if result.returncode > 0 else 1
            emit('Synthetic host exceeded its own 10.667 ms deadline; retain this failed '
                 'attempt and repeat the unchanged strict diagnostic.')
    return 1


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--report', type=Path, required=True)
    parser.add_argument('command', nargs=argparse.REMAINDER)
    args = parser.parse_args()
    command = args.command[1:] if args.command[:1] == ['--'] else args.command
    if not command:
        parser.error('Pass the diagnostic command after --')
    raise SystemExit(run(command, args.report))
