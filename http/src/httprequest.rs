use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}
#[derive(Debug, PartialEq)]
pub enum Verion {
    V1_1,
    V2_0,
    Uninitialized,
}
#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}
#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Verion,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}
impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Verion::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headres = HashMap::new();
        let mut parsed_msg_body = " ";
        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headres.insert(key, value);
            } else if line.len() == 0 {
            } else {
                parsed_msg_body = line;
            }
        }
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headres,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}
impl From<&str> for Verion {
    fn from(s: &str) -> Verion {
        match s {
            "HTTP/1.1" => Verion::V1_1,
            _ => Verion::Uninitialized,
        }
    }
}
impl From<&str> for Method {
    fn from(s: &str) -> Method {
        //吧字符串传送进来，切片，匹配到get就返回get方法匹配到post就返回post，否则返回初始化
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}
fn process_req_line(s: &str) -> (Method, Resource, Verion) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let verison = words.next().unwrap();
    (
        method.into(),
        Resource::Path(resource.to_string()),
        verison.into(),
    )
}
fn process_header_line(s: &str) -> (String, String) {
    //应该传入一行以：分隔的 如
    /*
		Accept-Encoding: gzip, deflate, br
		Accept-Language: zh,en-US;q=0.9,en;q=0.8,da;q=0.7,pl;q=0.6,ga;q=0.5
		Cache-Control: no-cache
		Connection: keep-alive
    */
    let mut header_itesm = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_itesm.next() {
        key = k.trim().to_string();
    }
    if let Some(v) = header_itesm.next() {
       value = v.trim().to_string();
	  
    }
    (key, value)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }
    #[test]
    fn test_verion_into() {
        let v: Verion = "HTTP/1.1".into();
        assert_eq!(v, Verion::V1_1);
    }
	#[test]
	fn test_read_http(){
		let s=String::from("GET /greeting HTTP/1.1 \r\nAccept:text/html\r\nAccept-Encoding: gzip, deflate, br\r\nAccept-Language:zh\r\n\r\n");
		let mut headers_expectted:HashMap<String,String>=HashMap::new();
		headers_expectted.insert("Accept".into(),"text/html".into());
		headers_expectted.insert("Accept-Encoding".into(), "gzip, deflate, br".into());
		headers_expectted.insert("Accept-Language".into(), "zh".into());
		let req:HttpRequest=s.into();
		assert_eq!(Method::Get,req.method);
		assert_eq!(Verion::V1_1,req.version);
		assert_eq!(Resource::Path("/greeting".to_string()),req.resource);
		assert_eq!(headers_expectted,req.headers);
	}
}
