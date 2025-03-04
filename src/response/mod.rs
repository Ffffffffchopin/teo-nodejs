mod header_map;
pub(crate) mod response_or_promise;

use std::path::PathBuf;

use crate::object::{js_any_to_teo_object, value::teo_value_to_js_any};

use self::header_map::ReadWriteHeaderMap;
use napi::{Result, JsUnknown, Env, bindgen_prelude::{FromNapiValue, FromNapiRef}};
use teo::prelude::response::Response as TeoResponse;

#[napi(js_name = "Response")]
pub struct Response {
    pub(crate) teo_response: TeoResponse,
}

#[napi]
impl Response {

    #[napi]
    pub fn empty() -> Self {
        Self {
            teo_response: TeoResponse::empty()
        }
    }

    #[napi]
    pub fn string(content: String, content_type: String) -> Self {
        Self {
            teo_response: TeoResponse::string(content, &content_type.as_str())
        }
    }

    #[napi(ts_return_type = "Response")]
    pub fn teon(value: JsUnknown, env: Env) -> Result<Self> {
        let teo_value = js_any_to_teo_object(value, env)?.as_teon().unwrap().clone();
        let response = TeoResponse::teon(teo_value);
        Ok(Self {
            teo_response: response
        })
    }

    #[napi]
    pub fn html(content: String) -> Self {
        Self::string(content, "text/html".to_owned())
    }

    #[napi(ts_return_type = "Response")]
    pub fn data(value: JsUnknown, env: Env) -> Result<Self> {
        let teo_value = js_any_to_teo_object(value, env)?.as_teon().unwrap().clone();
        let response = TeoResponse::data(teo_value);
        Ok(Self {
            teo_response: response
        })
    }
    
    #[napi(js_name = "dataMeta", ts_return_type = "Response")]
    pub fn data_meta(data: JsUnknown, meta: JsUnknown, env: Env) -> Result<Self> {
        let teo_data = js_any_to_teo_object(data, env)?.as_teon().unwrap().clone();
        let teo_meta = js_any_to_teo_object(meta, env)?.as_teon().unwrap().clone();
        let response = TeoResponse::data_meta(teo_data, teo_meta);
        Ok(Self {
            teo_response: response
        })
    }

    // error
    
    #[napi(js_name = "file")]
    pub fn file(path: String) -> Self {
        let path_buf = PathBuf::from(path);
        Self {
            teo_response: TeoResponse::file(path_buf)
        }
    }

    #[napi(js_name = "redirect")]
    pub fn redirect(path: String) -> Self {
        Self {
            teo_response: TeoResponse::redirect(path)
        }
    }

    #[napi(js_name = "setCode")]
    pub fn set_code(&self, code: u16) {
        self.teo_response.set_code(code)
    }

    #[napi(js_name = "code")]
    pub fn code(&self) -> u16 {
        self.teo_response.code()
    }

    #[napi(js_name = "headers")]
    pub fn headers(&self) -> ReadWriteHeaderMap {
        ReadWriteHeaderMap {
            inner: self.teo_response.headers()
        }
    }

    #[napi]
    pub fn is_file(&self) -> bool {
        self.teo_response.body().is_file()
    }

    #[napi]
    pub fn is_text(&self) -> bool {
        self.teo_response.body().is_text()
    }

    #[napi]
    pub fn is_empty(&self) -> bool {
        self.teo_response.body().is_empty()
    }

    #[napi]
    pub fn is_teon(&self) -> bool {
        self.teo_response.body().is_teon()
    }

    #[napi]
    pub fn get_text(&self) -> Option<String> {
        self.teo_response.body().as_text().cloned()
    }

    #[napi]
    pub fn get_teon(&self, env: Env) -> Result<JsUnknown> {
        Ok(match self.teo_response.body().as_teon() {
            None => env.get_undefined()?.into_unknown(),
            Some(value) => teo_value_to_js_any(value, &env)?
        })
    }

    #[napi]
    pub fn get_file(&self) -> Option<String> {
        match self.teo_response.body().as_file() {
            None => None,
            Some(path_buf) => Some(path_buf.to_str().unwrap().to_string()),
        }
    }
}

impl FromNapiValue for Response {
    unsafe fn from_napi_value(env: napi::sys::napi_env, napi_val: napi::sys::napi_value) -> Result<Self> {
        let response: &Response = Response::from_napi_ref(env, napi_val)?;
        Ok(Response { teo_response: response.teo_response.clone() })
    }
}