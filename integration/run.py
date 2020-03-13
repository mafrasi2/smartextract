#!/usr/bin/env python3

import argparse
import enum
import json
import re
import shutil
import subprocess as sp
import sys
import tempfile
from json import JSONDecoder, JSONDecodeError
from pathlib import Path
from pprint import pprint

import generate_desc

# from https://stackoverflow.com/a/50384432/625623
NOT_WHITESPACE = re.compile(r'[^\s]')
def decode_stacked_json(document, pos=0, decoder=JSONDecoder()):
    while True:
        match = NOT_WHITESPACE.search(document, pos)
        if not match:
            return
        pos = match.start()

        try:
            obj, pos = decoder.raw_decode(document, pos)
        except JSONDecodeError:
            raise
        yield obj


class UnpackError(Exception):
    def __init__(self, msg, log=None):
        super().__init__(msg)
        self.log = log
    def __str__(self):
        return f"{super().__str__()}. Output:\n{self.log}"

class CheckResult(enum.Enum):
    PASSED = enum.auto()
    FAILED = enum.auto()

def run(outdir, test, executable, cfg_file=None, verbose=False):
    shutil.copytree(test, outdir, dirs_exist_ok=True)
    (outdir / "description.json").unlink()

    cmd = [str(executable)]
    if cfg_file:
        cmd.extend(["-c", cfg_file])
    cmd.append(outdir)

    proc = sp.Popen(cmd, stdout=sp.PIPE, stderr=sp.STDOUT)
    stdout, _ = proc.communicate()
    if proc.returncode != 0:
        raise UnpackError(f"smartunpack exited with status {proc.returncode}", stdout.decode())

def check(outdir, should_desc, verbose=False):
    is_desc = generate_desc.describe(outdir)
    if is_desc == should_desc:
        return CheckResult.PASSED, is_desc
    else:
        return CheckResult.FAILED, is_desc

def run_test(test, executable, cfg_file=None, verbose=False):
    with (test / "description.json").open() as f:
        desc = json.load(f)
    with tempfile.TemporaryDirectory() as outdir:
        outdir = Path(outdir)
        run(outdir, test, executable, cfg_file=cfg_file, verbose=verbose)
        return check(outdir, desc, verbose=verbose)

def get_executable():
    msgs = sp.check_output(["cargo", "build", "--quiet", "--message-format=json"])
    for msg in decode_stacked_json(msgs.decode()):
        if "executable" in msg and msg["executable"]:
            return msg["executable"]

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run the smartunpack integration tests")
    parser.add_argument("tests", nargs="*", help="test names (if empty all tests are run)")
    parser.add_argument("-v", "--verbose", action="store_true", help="use verbose output")
    args = parser.parse_args()

    tests = [Path(test) for test in args.tests]
    if len(tests) == 0:
        test_root = Path(__file__).parent
        tests = [entry for entry in test_root.iterdir() if entry.is_dir() and entry.name != "__pycache__"]

    executable = get_executable()

    with tempfile.NamedTemporaryFile(prefix="smartunpack", suffix=".json") as cfg_file:
        shutil.copyfile(Path(__file__).parent / "smartunpack.json", cfg_file.name)

        all_passed = True
        for test in tests:
            print(f"Testing {test.name}...", end="")
            sys.stdout.flush()
            try:
                result, desc = run_test(test, executable, cfg_file=cfg_file.name, verbose=args.verbose)
                if result == CheckResult.PASSED:
                    print("PASSED")
                else:
                    print("FAILED")
                    if args.verbose:
                        print("error: check failed")
                        pprint(desc)
            except UnpackError as e:
                all_passed = False
                print("FAILED")
                if args.verbose:
                    print(e)
