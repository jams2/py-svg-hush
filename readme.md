# py-svg-hush

py-svg-hush is a Python package for sanitizing SVG files using the [svg-hush](https://github.com/cloudflare/svg-hush) Rust library.

## Installation

py-svg-hush requires Python 3.7 or later. You can install it using pip:

``` bash
pip install py_svg_hush
```

## Usage

``` python
from py_svg_hush import filter_svg

svg_bytes = b"""
<svg xmlns="http://www.w3.org/2000/svg">
    <script>alert('Malicious script')</script>
</svg>
"""

keep_data_url_mime_types = {
    "image": ["jpeg", "png", "gif"],
}

sanitized_svg = filter_svg(svg_bytes, keep_data_url_mime_types)
```


### filter_svg

The `filter_svg` function sanitizes SVG data by removing potentially malicious elements and attributes, as well as restricting the allowed MIME types of data: URLs in the SVG. It is a simple wrapper around the underlying Rust function.

#### Parameters

- `svg (bytes)`: The SVG data to be filtered.
- `keep_data_url_mime_types (Optional[Dict[str, List[str]]])`: A dictionary that maps MIME types to lists of allowed subtypes for data: URLs in the SVG. If a data: URL's MIME type and subtype are not in the dictionary it will be dropped. If this parameter is None, all data: URLs are dropped.

#### Returns

- `(bytes)`: The sanitized SVG data.

#### Raises

- `ValueError`: If there is an error processing the SVG data.
- `TypeError`: If any of the arguments are of the wrong type.


## Development

To set up a development environment for py-svg-hush, you can use the following steps:

Clone the repository:

``` bash
git clone git@github.com:jams2/py-svg-hush.git
```

Install development dependencies:

``` bash
pip install .[dev,testing]
```

Build the Rust library, resulting in a Python module:

``` bash
maturin develop
```

Run tests:

``` bash
pytest
```
