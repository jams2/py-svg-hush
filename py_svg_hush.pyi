from typing import Dict, List, Optional

def filter_svg(
    svg: bytes, keep_data_url_mime_types: Optional[Dict[str, List[str]]]
) -> bytes: ...
