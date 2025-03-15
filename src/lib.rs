use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::collections::HashMap;
use svg_hush::data_url_filter::DataUrl;
use svg_hush::data_url_filter::DataUrlFilterResult;
use svg_hush::*;

fn get_data_url_filter(
    keep_data_url_mime_types: Option<HashMap<String, Vec<String>>>,
) -> impl Fn(&DataUrl) -> DataUrlFilterResult {
    move |data_url: &DataUrl| -> DataUrlFilterResult {
        match &keep_data_url_mime_types {
            Some(mapping) => {
                let mime_type = data_url.mime_type();
                let subtype = mime_type.subtype.as_str();
                if let Some(subtypes) = mapping.get(&mime_type.type_) {
                    for allowed_subtype in subtypes {
                        if allowed_subtype == subtype {
                            return DataUrlFilterResult::Keep;
                        }
                    }
                }
                DataUrlFilterResult::Drop
            }
            None => DataUrlFilterResult::Drop,
        }
    }
}

#[pyfunction]
#[pyo3(
    signature = (svg, /, keep_data_url_mime_types=None),
    text_signature="(svg, /, keep_data_url_mime_types=None)"
)]
fn filter_svg(
    py: Python,
    svg: &[u8],
    keep_data_url_mime_types: Option<HashMap<String, Vec<String>>>,
) -> PyResult<Py<PyBytes>> {
    let mut filter = Filter::new();
    filter.set_data_url_filter(get_data_url_filter(keep_data_url_mime_types));
    let mut out: Vec<u8> = Vec::new();
    match filter.filter(svg, &mut out) {
        Ok(_) => Ok(PyBytes::new(py, &out).into()),
        Err(e) => Err(PyValueError::new_err(e.to_string())),
    }
}

#[pymodule]
fn py_svg_hush(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(filter_svg, m)?)?;
    Ok(())
}
