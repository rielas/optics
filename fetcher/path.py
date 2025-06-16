def get_domain(url: str) -> str:
    """Extract domain from URL."""
    if url.startswith(("http://", "https://")):
        url = url.split("://", 1)[1]

    domain = url.split("/")[0]
    return domain


def get_path(url: str) -> str:
    """Convert URL to file path by removing protocol and domain.

    Example: "https://en.wikipedia.org/wiki/Abbasiya" -> "wiki/Abbasiya.html"
    """

    if url.startswith(("http://", "https://")):
        url = url.split("://", 1)[1]

    parts = url.split("/")

    if len(parts) > 1:
        path_parts = parts[1:]

        if path_parts and path_parts[-1] == "":
            path_parts[-1] = "index.html"
            return "/".join(path_parts)

        return "/".join(path_parts) + ".html"

    return ""
