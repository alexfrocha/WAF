use std::collections::HashMap;

pub fn format_http_request_to_json(request: &str) -> String {
    let mut lines = request.lines();
    
    let request_line = lines.next().unwrap_or("").trim();
    
    let mut headers = HashMap::new();
    let mut is_header_section = false;
    
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            is_header_section = true;
            continue;
        }
        if is_header_section {
            continue; 
        }
        if let Some((key, value)) = line.split_once(':') {
            headers.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    
    let mut json_string = String::new();
    json_string.push_str("{\n");
    
    json_string.push_str(&format!("  \"request_line\": \"{}\",\n", request_line));
    
    json_string.push_str("  \"headers\": {\n");
    let mut header_iter = headers.iter().peekable();
    while let Some((key, value)) = header_iter.next() {
        json_string.push_str(&format!("    \"{}\": \"{}\"", key, value));
        if header_iter.peek().is_some() {
            json_string.push_str(",\n");
        } else {
            json_string.push_str("\n");
        }
    }
    json_string.push_str("  }\n");
    json_string.push_str("}");
    
    json_string
}

pub fn format_http_response_to_json(response: &str) -> String {
    let mut lines = response.lines();
    
    let status_line = lines.next().unwrap_or("").trim();
    
    let mut headers = HashMap::new();
    let mut is_header_section = false;
    
    for line in lines.by_ref() {
        let line = line.trim();
        if line.is_empty() {
            is_header_section = true;
            break; 
        }
        if !is_header_section {
            if let Some((key, value)) = line.split_once(':') {
                headers.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    }
    
    let body: String = lines.collect::<Vec<&str>>().join("\n");
    
    let mut json_string = String::new();
    json_string.push_str("{\n");
    
    json_string.push_str(&format!("  \"status_line\": \"{}\",\n", status_line));
    
    json_string.push_str("  \"headers\": {\n");
    let mut header_iter = headers.iter().peekable();
    while let Some((key, value)) = header_iter.next() {
        json_string.push_str(&format!("    \"{}\": \"{}\"", key, value));
        if header_iter.peek().is_some() {
            json_string.push_str(",\n");
        } else {
            json_string.push_str("\n");
        }
    }
    json_string.push_str("  },\n");
    
    json_string.push_str("  \"value\": ");
    if !body.is_empty() {
        json_string.push_str(&format!("\"{}\"\n", body.replace("\"", "\\\"")));
    } else {
        json_string.push_str("\"\"\n");
    }
    
    json_string.push_str("}");
    
    json_string
}