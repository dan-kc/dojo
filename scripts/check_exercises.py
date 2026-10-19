#!/usr/bin/env python3
"""Check the exercise/deck bijection and optionally test complete reference answers.

References are built in a temporary crate, never copied over the learner's work.
Requires Python 3.10+; --references also requires cargo, rustc and a C linker.
"""
import argparse
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def check(notes):
    manifest = json.loads((ROOT / 'exercises.json').read_text())
    entries = manifest['exercises']
    sources = {str(p.relative_to(ROOT)) for p in (ROOT / 'src').glob('*/*.rs') if p.name != 'mod.rs'}
    refs = {str(p.relative_to(ROOT)) for p in (ROOT / 'src').glob('*/*.md')}
    assert len(entries) == len({e['id'] for e in entries}), 'Duplicate exercise IDs'
    assert len(entries) == len({e['card'] for e in entries}), 'Duplicate card mappings'
    assert sources == {e['source'] for e in entries}, 'Source/manifest mismatch'
    assert refs == {e['reference'] for e in entries}, 'Reference/manifest mismatch'
    if notes:
        actual = {str(p.relative_to(notes)) for p in (notes / 'cards/dojo').rglob('*.md')}
        assert actual == {e['card'] for e in entries}, 'Deck/manifest mismatch'
    for e in entries:
        source = (ROOT / e['source']).read_text()
        assert e['source'] == f"src/{e['id']}.rs"
        assert e['reference'] == f"src/{e['id']}.md"
        assert '#[cfg(test)]' in source, f"Missing tests: {e['id']}"
        assert re.search(r'#\[(?:tokio::)?test(?:\(|\])', source), f"No tests: {e['id']}"
        module = e['id'].split('/')[-1]
        modfile = (ROOT / e['source']).parent / 'mod.rs'
        assert re.search(rf'(?m)^pub mod {re.escape(module)};', modfile.read_text()), f"Unregistered: {e['id']}"
        reference = reference_code(e)
        assert not re.search(r'\b(?:todo|unimplemented)!', reference), f"Incomplete reference: {e['id']}"
        if notes:
            card = (notes / e['card']).read_text()
            front = re.search(r'<!-- FRONT:BEGIN -->(.*?)<!-- FRONT:END -->', card, re.S)
            assert front and front[1].strip() == e['id'], f"Wrong front: {e['card']}"
            assert '<!-- BACK:BEGIN -->' not in card, f"Unexpected answer back: {e['card']}"
            history = re.search(r'<!-- HISTORY:BEGIN -->(.*?)<!-- HISTORY:END -->', card, re.S)
            if history:
                for line in history[1].splitlines():
                    if re.match(r'\|\s*\d', line):
                        assert re.fullmatch(r'\|\s*\d{4}-\d{2}-\d{2}\s*\|\s*[1-4]\s*\|', line), f"Invalid history: {e['card']}"
    print(f"Validated {len(entries)} exercises, references and {'cards' if notes else 'module registrations'}.", flush=True)
    return entries


def reference_code(entry):
    blocks = re.findall(r'```rust\s*\n(.*?)```', (ROOT / entry['reference']).read_text(), re.S)
    assert len(blocks) == 1, f"Expected one complete reference: {entry['id']}"
    return blocks[0]


def references(entries, timeout):
    # Keep artifacts cached but isolate source and manifests from the learner repository.
    env = dict(os.environ)
    env.setdefault('CARGO_TARGET_DIR', str(ROOT / 'target/reference-check'))
    with tempfile.TemporaryDirectory(prefix='dojo-references-') as tmp:
        dest = Path(tmp)
        (dest / 'Cargo.toml').write_text((ROOT / 'Cargo.toml').read_text().replace('name = \"dojo\"', 'name = \"dojo_reference_check\"', 1))
        (dest / 'Cargo.lock').write_text((ROOT / 'Cargo.lock').read_text().replace('name = \"dojo\"', 'name = \"dojo_reference_check\"', 1))
        shutil.copytree(ROOT / 'src', dest / 'src')
        for e in entries:
            text = (ROOT / e['source']).read_text()
            _, separator, tests = text.partition('#[cfg(test)]')
            (dest / e['source']).write_text(reference_code(e) + '\n' + separator + tests)
        build = subprocess.run(['cargo', 'test', '--locked', '--lib', '--no-run', '--message-format=json'], cwd=dest, env=env, text=True, stdout=subprocess.PIPE)
        if build.returncode:
            for line in build.stdout.splitlines():
                try:
                    item = json.loads(line)
                    if item.get('reason') == 'compiler-message':
                        print(item['message'].get('rendered', ''))
                except json.JSONDecodeError:
                    print(line)
            raise SystemExit(build.returncode)
        executable = None
        for line in build.stdout.splitlines():
            item = json.loads(line)
            if item.get('reason') == 'compiler-artifact' and item.get('executable'):
                executable = item['executable']
        assert executable, 'No reference test executable produced'
        failures = []
        total = 0
        for e in entries:
            module = e['id'].replace('/', '::') + '::'
            try:
                result = subprocess.run([executable, module, '--test-threads=4'], cwd=dest, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=timeout)
                count = re.search(r'test result: .*? (\d+) passed;', result.stdout)
                if result.returncode or not count or int(count[1]) == 0:
                    failures.append(e['id'])
                    print(result.stdout)
                else:
                    total += int(count[1])
            except subprocess.TimeoutExpired as err:
                failures.append(e['id'])
                print(f"TIMEOUT {e['id']} after {timeout}s\n{err.stdout or ''}")
        if failures:
            raise SystemExit('Reference failures: ' + ', '.join(failures))
        print(f'Passed {total} reference tests across all {len(entries)} exercises.')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--notes', type=Path, default=ROOT.parent / 'notes', help='Notes repository; default sibling notes')
    parser.add_argument('--without-notes', action='store_true', help='Validate Dojo alone')
    parser.add_argument('--references', action='store_true', help='Compile and execute each reference with its exercise tests')
    parser.add_argument('--timeout', type=int, default=30, help='Deadlock guard per exercise in seconds')
    args = parser.parse_args()
    entries = check(None if args.without_notes else args.notes)
    if args.references:
        references(entries, args.timeout)


if __name__ == '__main__':
    main()
