import re
import unittest
import sys
import os

# Add the parent directory to the path so we can import from fetcher
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from fetcher.path import get_path


def is_valid_filename(filename: str) -> bool:
    """Check if filename is valid for the current operating system"""

    if not filename or filename.strip() == "":
        return False

    invalid_chars = r'[<>"\\|?*\x00-\x1f]'

    if re.search(invalid_chars, filename):
        return False

    return True


class TestPagename(unittest.TestCase):
    def test_wikipedia_url(self):
        """Test converting Wikipedia URL to path."""
        url = "https://en.wikipedia.org/wiki/Abbasiya"
        expected = "wiki/Abbasiya.html"
        self.assertEqual(get_path(url), expected)

    def test_http_url(self):
        """Test HTTP (non-HTTPS) URL conversion."""
        url = "http://en.wikipedia.org/wiki/Cretaceous"
        expected = "wiki/Cretaceous.html"
        self.assertEqual(get_path(url), expected)

    def test_nested_path(self):
        """Test URL with nested path."""
        url = "https://en.wikipedia.org/wiki/Category:Jurassic_crustaceans"
        expected = "wiki/Category:Jurassic_crustaceans.html"
        self.assertTrue(is_valid_filename(expected))
        self.assertEqual(get_path(url), expected)

    def test_user_page(self):
        """Test user page URL."""
        url = "https://en.wikipedia.org/wiki/User:Lavalizard101"
        expected = "wiki/User:Lavalizard101.html"
        self.assertTrue(is_valid_filename(expected))
        self.assertEqual(get_path(url), expected)

    def test_domain_only(self):
        """Test URL with domain only."""
        url = "https://en.wikipedia.org/wiki/"
        expected = "wiki/index.html"
        self.assertEqual(get_path(url), expected)


if __name__ == "__main__":
    unittest.main()
