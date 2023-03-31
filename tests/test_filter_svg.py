from py_svg_hush import filter_svg

SVG_NO_CONTENT = (
    b'<?xml version="1.0" encoding="utf-8"?>'
    b'\n<svg xmlns="http://www.w3.org/2000/svg"/>'
)


def test_script_tags_stripped():
    svg = (
        b'<svg xmlns="http://www.w3.org/2000/svg"><script>alert("hello")</script></svg>'
    )
    assert filter_svg(svg) == SVG_NO_CONTENT


def test_allowed_mime_types_kept():
    svg = (
        b'<?xml version="1.0" encoding="utf-8"?>'
        b'\n<svg xmlns="http://www.w3.org/2000/svg">'
        b'\n<image href="data:image/png;base64,iVBORw0KGg=="/>'
        b'\n</svg>'
    )
    assert filter_svg(svg, keep_data_url_mime_types={"image": ["png"]}) == svg


def test_data_urls_removed():
    svg = (
        b'<svg xmlns="http://www.w3.org/2000/svg">'
        b'<a href="data:text/javascript;base64,PHNjcmlwdD5hbGVydCgnSGVsbG8gV29ybGQhJyk7PC9zY3JpcHQ+Cg=="></a>'
        b'</svg>'
    )
    assert filter_svg(svg) == (
        b'<?xml version="1.0" encoding="utf-8"?>'
        b'\n<svg xmlns="http://www.w3.org/2000/svg">\n  <a/>\n</svg>'
    )
