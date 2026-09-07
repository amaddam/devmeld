"""Tests use isolated files, never the repository's installed skills."""

from pathlib import Path
import tempfile
import unittest

from sync_skills import COMMANDS, sync


class SyncSkillsTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        for name in COMMANDS:
            source = self.source(name)
            source.parent.mkdir(parents=True, exist_ok=True)
            source.write_text(f"accepted {name} command\n", encoding="utf-8")

    def source(self, name):
        return self.root / ".specify/templates/overrides" / f"{name}.md"

    def target(self, name):
        return self.root / ".agents/skills" / f"speckit-{name}" / "SKILL.md"

    def test_check_reports_missing_output_without_writing(self):
        self.assertFalse(sync(self.root))
        self.assertFalse((self.root / ".agents").exists())

    def test_write_restores_only_owned_outputs_and_is_idempotent(self):
        unrelated = self.root / "unrelated.md"
        unrelated.write_text("keep me", encoding="utf-8")
        self.assertTrue(sync(self.root, write=True))
        self.assertTrue(sync(self.root))
        self.target("tasks").write_text("upstream refresh\n", encoding="utf-8")
        self.assertFalse(sync(self.root))
        self.assertEqual(self.target("tasks").read_text(encoding="utf-8"), "upstream refresh\n")
        self.assertTrue(sync(self.root, write=True))
        for name in COMMANDS:
            self.assertEqual(
                self.target(name).read_text(encoding="utf-8"),
                self.source(name).read_text(encoding="utf-8"),
            )
        self.assertEqual(unrelated.read_text(encoding="utf-8"), "keep me")

    def test_missing_source_does_not_partially_write(self):
        self.source("implement").unlink()
        with self.assertRaises(FileNotFoundError):
            sync(self.root, write=True)
        self.assertFalse(self.target("tasks").exists())

    def test_crlf_and_lf_are_equivalent(self):
        sync(self.root, write=True)
        self.target("tasks").write_bytes(b"accepted tasks command\r\n")
        self.assertTrue(sync(self.root))

    def test_redirected_target_cannot_overwrite_an_unrelated_file(self):
        unrelated = self.root / "keep.md"
        unrelated.write_text("keep me", encoding="utf-8")
        target = self.target("tasks")
        target.parent.mkdir(parents=True)
        try:
            target.symlink_to(unrelated)
        except OSError as error:
            self.skipTest(f"Symlinks unavailable: {error}")
        with self.assertRaises(ValueError):
            sync(self.root, write=True)
        self.assertEqual(unrelated.read_text(encoding="utf-8"), "keep me")


if __name__ == "__main__":
    unittest.main()
