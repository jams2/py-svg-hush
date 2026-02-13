def filter_svg(
    svg: bytes,
    /,
    keep_data_url_mime_types: dict[str, list[str]] | None = None,
) -> bytes: ...
