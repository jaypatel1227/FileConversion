mod service {
    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    type ConversionFunction = fn(String) -> String; // this is the type signature of a conversion function

    // this is the mapping that is used to define the available conversions that are availiable
    static AVAILABLE_CONVERSIONS: Lazy<HashMap<String, ConversionFunction>> = Lazy::new(|| {
        let mut map = HashMap::new();
        // Use a call like below to register a new conversion service
        map.insert("docx_to_pdf".to_string(), docx_to_pdf as ConversionFunction);
        map
    });

    pub fn call(func_name: String, file_name: String) -> String {
        if let Some(conversion_func) = AVAILABLE_CONVERSIONS.get(&func_name) {
            let result = conversion_func(file_name);
            println!("Conversion result: {}", result);
            return result;
        }
        return "".to_string();
    }

    pub fn docx_to_pdf(file: String) -> String {
        return "TODO".to_string();
    }
}
